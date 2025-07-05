use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

use crate::{Game, GameState, LudoError, GAME};

#[vrf]
#[derive(Accounts)]
pub struct RollDiceDelegateCtx<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
    /// CHECK: The oracle queue
    #[account(mut, address = ephemeral_vrf_sdk::consts::DEFAULT_EPHEMERAL_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
}

impl<'info> RollDiceDelegateCtx<'info> {
    pub fn roll_dice_delegate(&mut self, client_seed: u8) -> Result<()> {
        let game: &mut Account<'info, Game> = &mut self.game;

        require!(
            game.game_state == GameState::RollDice,
            LudoError::WrongGameState
        );

        require!(
            game.cur_player_key() == self.player.key(),
            LudoError::WrongPlayer
        );

        game.game_state = GameState::RollingDice;

        msg!("Requesting randomness...");
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: self.player.key(),
            oracle_queue: self.oracle_queue.key(),
            callback_program_id: crate::ID,
            callback_discriminator: crate::instruction::CallbackRollDice::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            // Specify any account that is required by the callback
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: game.key(),
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
pub struct CallbackRollDiceCtx<'info> {
    /// This check ensure that the vrf_program_identity (which is a PDA) is a singer
    /// enforcing the callback is executed by the VRF program through CPI
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut)]
    pub game: Account<'info, Game>,
}

impl<'info> CallbackRollDiceCtx<'info> {
    pub fn callback_roll_dice(&mut self, randomness: [u8; 32]) -> Result<()> {
        let roll = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 1, 6);
        msg!("Consuming random number: {:?}", roll);

        let game = &mut self.game;

        let mut skip_turn = true;

        if !(roll == 6 && game.six_count == 2) {
            for token in game.token_positions[game.cur_player as usize].iter() {
                if *token == -1 && roll != 6 || *token == 56 {
                    continue;
                }
                if *token + roll as i8 <= 56 {
                    skip_turn = false;
                }
            }
        }

        if skip_turn {
            game.six_count = 0;
            game.next_player();
            game.game_state = GameState::RollDice;
        } else {
            game.current_roll = roll;
            game.game_state = GameState::Move;
        }
        Ok(())
    }
}
