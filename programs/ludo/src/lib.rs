#![allow(deprecated)]
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{commit, delegate, ephemeral};
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

declare_id!("Ab2xsYDGv4GKKJc2wuKiGeqtXhP6ukJFNAHXvoCBVdHG");

pub const GAME: &[u8] = b"ludo_game";

pub const SAFE_POSITIONS: [i8; 8] = [0, 13, 26, 39, 8, 21, 34, 47];

#[ephemeral]
#[program]
pub mod ludo {
    use super::*;

    pub fn create_game(
        ctx: Context<CreateGame>,
        seed: u64,
        num_players: u8,
        color: Colors,
    ) -> Result<()> {
        ctx.accounts.game.set_inner(Game {
            seed,
            bump: ctx.bumps.game,
            num_players,
            cur_player: 1,
            token_positions: [[-1; 4]; 4],
            game_state: GameState::NotStarted,
            current_roll: 0,
            six_count: 0,
            players: [Pubkey::default(); 4],
            winner: Pubkey::default(),
        });
        ctx.accounts.game.players[color as usize] = ctx.accounts.player.key();
        Ok(())
    }

    pub fn join_game(ctx: Context<JoinGame>, color: Colors) -> Result<()> {
        let game = &mut ctx.accounts.game;

        if game.game_state != GameState::NotStarted {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        if game.cur_player + 1 == game.num_players {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        let player = &ctx.accounts.player;

        if game.players.contains(&player.key()) {
            return Err(ProgramError::InvalidInstructionData.into());
        }
        if game.players[color as usize] != Pubkey::default() {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        game.players[color as usize] = player.key();

        game.cur_player += 1;

        Ok(())
    }

    pub fn join_and_start_game(
        ctx: Context<JoinStartGame>,
        color: Colors,
        client_seed: u8,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;

        if game.game_state != GameState::NotStarted {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        if game.cur_player + 1 != game.num_players {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        let player = &ctx.accounts.player;

        if game.players.contains(&player.key()) {
            return Err(ProgramError::InvalidInstructionData.into());
        }
        if game.players[color as usize] != Pubkey::default() {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        game.players[color as usize] = player.key();

        game.cur_player = 0;
        game.game_state = GameState::Starting;

        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: ctx.accounts.player.key(),
            oracle_queue: ctx.accounts.oracle_queue.key(),
            callback_program_id: ID,
            callback_discriminator: instruction::CallbackStartGame::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            // Specify any account that is required by the callback
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: ctx.accounts.game.key(),
                is_signer: false,
                is_writable: true,
            }]),
            ..Default::default()
        });
        ctx.accounts
            .invoke_signed_vrf(&ctx.accounts.player.to_account_info(), &ix)?;

        Ok(())
    }

    pub fn callback_start_game(
        ctx: Context<CallbackStartGameCtx>,
        randomness: [u8; 32],
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let rnd_u8 =
            ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 0, game.num_players - 1);
        msg!("Consuming random number: {:?}", rnd_u8);

        if game.players[game.cur_player as usize] == Pubkey::default() {
            next_player(game);
        }

        for _ in 0..rnd_u8 {
            next_player(game);
        }

        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn roll_dice(ctx: Context<DoRollDiceCtx>, client_seed: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;

        if game.game_state != GameState::RollDice {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        let cur_player = game.cur_player;

        let cur_player_key = game.players[cur_player as usize];
        if cur_player_key != ctx.accounts.player.key() {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        game.game_state = GameState::RollingDice;

        msg!("Requesting randomness...");
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: ctx.accounts.player.key(),
            oracle_queue: ctx.accounts.oracle_queue.key(),
            callback_program_id: ID,
            callback_discriminator: instruction::CallbackRollDice::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            // Specify any account that is required by the callback
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: ctx.accounts.game.key(),
                is_signer: false,
                is_writable: true,
            }]),
            ..Default::default()
        });
        ctx.accounts
            .invoke_signed_vrf(&ctx.accounts.player.to_account_info(), &ix)?;
        Ok(())
    }

    pub fn callback_roll_dice(
        ctx: Context<CallbackRollDiceCtx>,
        randomness: [u8; 32],
    ) -> Result<()> {
        let rnd_u8 = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 1, 6);
        msg!("Consuming random number: {:?}", rnd_u8);

        let game = &mut ctx.accounts.game;
        game.current_roll = rnd_u8;
        game.game_state = GameState::Move;
        Ok(())
    }

    pub fn token_into_play(ctx: Context<TokenIntoPlay>, token_num: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let cur_player = game.cur_player;

        let cur_player_key = game.players[cur_player as usize];
        if cur_player_key != ctx.accounts.player.key() {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        if game.current_roll != 6 {
            return Err(ProgramError::InvalidInstructionData.into());
        }
        if game.token_positions[cur_player as usize][token_num as usize] != -1 {
            return Err(ProgramError::InvalidInstructionData.into());
        }
        game.token_positions[cur_player as usize][token_num as usize] = 0;

        if game.six_count == 2 {
            game.six_count = 0;
            next_player(game);
        } else {
            game.six_count += 1;
        }

        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn skip_move(ctx: Context<Move>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let cur_player = game.cur_player;

        let cur_player_key = game.players[cur_player as usize];
        if cur_player_key != ctx.accounts.player.key() {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        for token in game.token_positions[cur_player as usize].iter() {
            if *token == -1 || *token == 56 {
                continue;
            }
            if *token + game.current_roll as i8 <= 56 {
                return Err(ProgramError::InvalidInstructionData.into());
            }
        }

        next_player(game);
        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn make_move(ctx: Context<Move>, token_num: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let cur_player = game.cur_player;

        let cur_player_key = game.players[cur_player as usize];
        if cur_player_key != ctx.accounts.player.key() {
            return Err(ProgramError::MissingRequiredSignature.into());
        }

        let cur_position = game.token_positions[cur_player as usize][token_num as usize];
        if cur_position == -1 {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        let new_position = cur_position + game.current_roll as i8;

        if new_position > 56 {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        game.token_positions[cur_player as usize][token_num as usize] = new_position;
        if game.token_positions[cur_player as usize]
            .iter()
            .all(|&x| x == 56)
        {
            game.game_state = GameState::Finished;
            game.winner = ctx.accounts.player.key();
            return Ok(());
        }

        if !SAFE_POSITIONS.contains(&new_position) && new_position < 51 {
            'outer: for defender in 0..4 {
                if cur_player == defender {
                    continue;
                }
                if game.players[defender as usize] == Pubkey::default() {
                    continue;
                }
                for i in 0..4 {
                    let def_position = game.token_positions[defender as usize][i];
                    if def_position == -1 {
                        continue;
                    }
                    let go_home = if cur_player < defender {
                        new_position + (cur_player as i8 - defender as i8 + 4) * 13 == def_position
                    } else {
                        def_position + (defender as i8 - cur_player as i8 + 4) * 13 == new_position
                    };
                    if go_home {
                        // check for blocks
                        for j in 0..4 {
                            if j == i {
                                continue;
                            }
                            if game.token_positions[defender as usize][i]
                                == game.token_positions[defender as usize][j]
                            {
                                game.token_positions[cur_player as usize][token_num as usize] = -1;
                                break 'outer;
                            }
                        }
                        game.token_positions[defender as usize][i] = -1;
                        break 'outer;
                    }
                }
            }
        }

        if game.current_roll == 6 {
            if game.six_count == 2 {
                game.six_count = 0;
                next_player(game);
            } else {
                game.six_count += 1;
            }
        } else {
            game.six_count = 0;
            next_player(game);
        }
        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        ctx.accounts.delegate_game(
            &ctx.accounts.user,
            &[GAME, &ctx.accounts.game.seed.to_le_bytes()],
            DelegateConfig::default(),
        )?;
        Ok(())
    }

    pub fn undelegate(ctx: Context<Undelegate>) -> Result<()> {
        if ctx.accounts.game.game_state != GameState::Finished {
            return Err(ProgramError::InvalidInstructionData.into());
        }
        commit_and_undelegate_accounts(
            &ctx.accounts.payer,
            vec![&ctx.accounts.game.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(init, payer = player, space = 8 + Game::INIT_SPACE, seeds = [GAME, seed.to_le_bytes().as_ref()], bump)]
    pub game: Account<'info, Game>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}

#[vrf]
#[derive(Accounts)]
pub struct JoinStartGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
    /// CHECK: The oracle queue
    #[account(mut, address = ephemeral_vrf_sdk::consts::DEFAULT_EPHEMERAL_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
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

#[vrf]
#[derive(Accounts)]
pub struct DoRollDiceCtx<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
    /// CHECK: The oracle queue
    #[account(mut, address = ephemeral_vrf_sdk::consts::DEFAULT_EPHEMERAL_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
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

#[derive(Accounts)]
pub struct TokenIntoPlay<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}

#[derive(Accounts)]
pub struct Move<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}

#[delegate]
#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK The pda to delegate
    #[account(mut, del, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}

#[commit]
#[derive(Accounts)]
pub struct Undelegate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}


#[account]
#[derive(InitSpace)]
pub struct Game {
    pub seed: u64,
    pub bump: u8,
    pub num_players: u8,
    pub cur_player: u8,
    pub token_positions: [[i8; 4]; 4],
    pub game_state: GameState,
    pub current_roll: u8,
    pub six_count: u8,
    pub players: [Pubkey; 4],
    pub winner: Pubkey, // ??? is there one winner or multiple???
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum GameState {
    NotStarted,
    Starting,
    RollDice,
    RollingDice,
    Move,
    Finished,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum Colors {
    Red = 0,
    Green = 1,
    Yellow = 2,
    Blue = 3,
}

pub fn next_player(game: &mut Game) {
    loop {
        game.cur_player = (game.cur_player + 1) % 4;
        if game.players[game.cur_player as usize] != Pubkey::default() {
            break;
        }
    }
}
