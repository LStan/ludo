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
        require!(
            num_players == 2 || num_players == 3 || num_players == 4,
            LudoError::InvalidNumPlayers
        );
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

    pub fn cancel_game(ctx: Context<CancelGame>, color: Colors) -> Result<()> {
        let game = &mut ctx.accounts.game;

        require!(
            game.game_state == GameState::NotStarted,
            LudoError::GameAlreadyStarted
        );

        require!(game.cur_player == 1, LudoError::AnotherPlayerAlreadyJoined);

        require!(
            game.players[color as usize] == ctx.accounts.player.key(),
            LudoError::WrongPlayer
        );
        Ok(())
    }

    pub fn join_game(ctx: Context<JoinGame>, color: Colors) -> Result<()> {
        let game = &mut ctx.accounts.game;

        require!(
            game.game_state == GameState::NotStarted,
            LudoError::GameAlreadyStarted
        );

        require!(
            game.cur_player + 1 < game.num_players,
            LudoError::NeedToRunJoinAndStart
        );

        let player = &ctx.accounts.player;

        require!(
            !game.players.contains(&player.key()),
            LudoError::PlayerAlreadyJoined
        );

        require!(
            game.players[color as usize] == Pubkey::default(),
            LudoError::ColorAlreadyTaken
        );

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

        require!(
            game.game_state == GameState::NotStarted,
            LudoError::GameAlreadyStarted
        );

        require!(
            game.cur_player + 1 == game.num_players,
            LudoError::NeedToRunJoin
        );

        let player = &ctx.accounts.player;

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
            game.next_player();
        }

        for _ in 0..rnd_u8 {
            game.next_player();
        }

        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn roll_dice_delegate(ctx: Context<RollDiceDelegateCtx>, client_seed: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;

        require!(
            game.game_state == GameState::RollDice,
            LudoError::WrongGameState
        );

        let cur_player = game.cur_player;

        require!(
            game.players[cur_player as usize] == ctx.accounts.player.key(),
            LudoError::WrongPlayer
        );

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
        require!(
            game.game_state == GameState::Move,
            LudoError::WrongGameState
        );

        let cur_player = game.cur_player;

        require!(
            game.players[cur_player as usize] == ctx.accounts.player.key(),
            LudoError::WrongPlayer
        );

        require!(game.current_roll == 6, LudoError::WrongMove);

        require!(game.six_count < 2, LudoError::WrongMove);
        require!(
            game.token_positions[cur_player as usize][token_num as usize] == -1,
            LudoError::WrongMove
        );

        game.token_positions[cur_player as usize][token_num as usize] = 0;

        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn skip_move(ctx: Context<Move>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        require!(
            game.game_state == GameState::Move,
            LudoError::WrongGameState
        );

        let cur_player = game.cur_player;

        require!(
            game.players[cur_player as usize] == ctx.accounts.player.key(),
            LudoError::WrongPlayer
        );

        if !(game.current_roll == 6 && game.six_count == 2) {
            for token in game.token_positions[cur_player as usize].iter() {
                if *token == -1 && game.current_roll != 6 || *token == 56 {
                    continue;
                }
                if *token + game.current_roll as i8 <= 56 {
                    return Err(LudoError::WrongMove.into());
                }
            }
        }

        game.six_count = 0;
        game.next_player();
        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn make_move(ctx: Context<Move>, token_num: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        require!(
            game.game_state == GameState::Move,
            LudoError::WrongGameState
        );

        let cur_player = game.cur_player;

        require!(
            game.players[cur_player as usize] == ctx.accounts.player.key(),
            LudoError::WrongPlayer
        );

        require!(
            (game.six_count == 2 && game.current_roll != 6) || game.six_count < 2,
            LudoError::WrongMove
        );

        let cur_position = game.token_positions[cur_player as usize][token_num as usize];

        require!(cur_position != -1, LudoError::WrongMove);

        let new_position = cur_position + game.current_roll as i8;

        require!(new_position <= 56, LudoError::WrongMove);

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
                for token in 0..4 {
                    let def_position = game.token_positions[defender as usize][token];
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
                        for token2 in 0..4 {
                            if token2 == token {
                                continue;
                            }
                            // block found
                            if game.token_positions[defender as usize][token]
                                == game.token_positions[defender as usize][token2]
                            {
                                game.token_positions[cur_player as usize][token_num as usize] = -1;
                                break 'outer;
                            }
                        }
                        // no blocks
                        game.token_positions[defender as usize][token] = -1;
                        break 'outer;
                    }
                }
            }
        }

        if game.current_roll == 6 {
            game.six_count += 1;
        } else {
            game.six_count = 0;
            game.next_player();
        }
        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn roll_dice_debug(ctx: Context<Move>, roll: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.current_roll = roll;
        game.game_state = GameState::Move;
        Ok(())
    }

    pub fn move_tile_debug(
        ctx: Context<Move>,
        color: Colors,
        token_num: u8,
        position: i8,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.token_positions[color as usize][token_num as usize] = position;
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
        require!(
            ctx.accounts.game.game_state == GameState::Finished,
            LudoError::GameNotFinished
        );
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
pub struct CancelGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump, close = player)]
    pub game: Account<'info, Game>,
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
pub struct RollDiceDelegateCtx<'info> {
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

impl Game {
    pub fn next_player(&mut self) {
        loop {
            self.cur_player = (self.cur_player + 1) % 4;
            if self.players[self.cur_player as usize] != Pubkey::default() {
                break;
            }
        }
    }
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

#[error_code]
pub enum LudoError {
    InvalidNumPlayers,
    AnotherPlayerAlreadyJoined,
    GameAlreadyStarted,
    WrongPlayer,
    NeedToRunJoinAndStart,
    NeedToRunJoin,
    PlayerAlreadyJoined,
    ColorAlreadyTaken,
    WrongGameState,
    WrongMove,
    GameNotFinished,
}
