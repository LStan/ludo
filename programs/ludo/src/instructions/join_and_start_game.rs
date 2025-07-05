use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

use crate::{Colors, Game, GameState, LudoError, GAME};

#[vrf]
#[derive(Accounts)]
pub struct JoinStartGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
    /// CHECK: The oracle queue
    #[account(mut, address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
}

impl<'info> JoinStartGame<'info> {
    pub fn join_and_start_game(&mut self, color: Colors, client_seed: u8) -> Result<()> {
        let game: &mut Account<'info, Game> = &mut self.game;

        require!(
            game.game_state == GameState::NotStarted,
            LudoError::GameAlreadyStarted
        );

        require!(
            game.cur_player + 1 == game.num_players,
            LudoError::NeedToRunJoin
        );

        let player = &self.player;

        require!(
            !game.players.contains(&player.key()),
            LudoError::PlayerAlreadyJoined
        );

        require!(
            game.players[color as usize] == Pubkey::default(),
            LudoError::ColorAlreadyTaken
        );

        game.players[color as usize] = player.key();

        game.cur_player = 0;
        game.game_state = GameState::Starting;

        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: self.player.key(),
            oracle_queue: self.oracle_queue.key(),
            callback_program_id: crate::ID,
            callback_discriminator: crate::instruction::CallbackStartGame::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            // Specify any account that is required by the callback
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: self.game.key(),
                is_signer: false,
                is_writable: true,
            }]),
            ..Default::default()
        });
        self.invoke_signed_vrf(&self.player.to_account_info(), &ix)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CallbackStartGameCtx<'info> {
    /// This check ensure that the vrf_program_identity (which is a PDA) is a singer
    /// enforcing the callback is executed by the VRF program through CPI
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut)]
    pub game: Account<'info, Game>,
}

impl<'info> CallbackStartGameCtx<'info> {
    pub fn callback_start_game(&mut self, randomness: [u8; 32]) -> Result<()> {
        let game = &mut self.game;
        let rnd_u8 =
            ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 0, game.num_players - 1);
        msg!("Consuming random number: {:?}", rnd_u8);

        if game.cur_player_key() == Pubkey::default() {
            game.next_player();
        }

        for _ in 0..rnd_u8 {
            game.next_player();
        }

        game.game_state = GameState::RollDice;
        Ok(())
    }
}
