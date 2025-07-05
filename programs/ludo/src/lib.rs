#![allow(deprecated)]
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::ephemeral;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ab2xsYDGv4GKKJc2wuKiGeqtXhP6ukJFNAHXvoCBVdHG");

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
        ctx.accounts
            .create_game(seed, num_players, color, &ctx.bumps)
    }

    pub fn cancel_game(ctx: Context<CancelGame>, color: Colors) -> Result<()> {
        ctx.accounts.cancel_game(color)
    }

    pub fn join_game(ctx: Context<JoinGame>, color: Colors) -> Result<()> {
        ctx.accounts.join_game(color)
    }

    pub fn join_and_start_game(
        ctx: Context<JoinStartGame>,
        color: Colors,
        client_seed: u8,
    ) -> Result<()> {
        ctx.accounts.join_and_start_game(color, client_seed)
    }

    pub fn callback_start_game(
        ctx: Context<CallbackStartGameCtx>,
        randomness: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.callback_start_game(randomness)
    }

    pub fn roll_dice_delegate(ctx: Context<RollDiceDelegateCtx>, client_seed: u8) -> Result<()> {
        ctx.accounts.roll_dice_delegate(client_seed)
    }

    pub fn callback_roll_dice(
        ctx: Context<CallbackRollDiceCtx>,
        randomness: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.callback_roll_dice(randomness)
    }

    pub fn token_into_play(ctx: Context<Move>, token_num: u8) -> Result<()> {
        ctx.accounts.token_into_play(token_num)
    }

    pub fn make_move(ctx: Context<Move>, token_num: u8) -> Result<()> {
        ctx.accounts.make_move(token_num)
    }

    pub fn join_and_start_game_debug(
        ctx: Context<Debug>,
        color: Colors,
        start_player: Colors,
    ) -> Result<()> {
        ctx.accounts.join_and_start_game_debug(color, start_player)
    }

    pub fn roll_dice_debug(ctx: Context<Debug>, roll: u8) -> Result<()> {
        ctx.accounts.roll_dice_debug(roll)
    }

    pub fn next_turn_debug(ctx: Context<Debug>) -> Result<()> {
        ctx.accounts.next_turn_debug()
    }

    pub fn move_token_debug(
        ctx: Context<Debug>,
        color: Colors,
        token_num: u8,
        position: i8,
    ) -> Result<()> {
        ctx.accounts.move_token_debug(color, token_num, position)
    }

    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        ctx.accounts.delegate()
    }

    pub fn undelegate(ctx: Context<Undelegate>) -> Result<()> {
        ctx.accounts.undelegate()
    }

    pub fn close_game(_ctx: Context<CloseGame>) -> Result<()> {
        Ok(())
    }
}
