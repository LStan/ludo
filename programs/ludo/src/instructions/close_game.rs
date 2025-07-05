use anchor_lang::prelude::*;

use crate::{Game, GAME};

#[derive(Accounts)]
pub struct CloseGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, close = player, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}
