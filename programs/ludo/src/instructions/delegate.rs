use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;

use crate::{Game, GAME};

#[delegate]
#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK The pda to delegate
    #[account(mut, del, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}

impl<'info> Delegate<'info> {
    pub fn delegate(&mut self) -> Result<()> {
        self.delegate_game(
            &self.user,
            &[GAME, &self.game.seed.to_le_bytes()],
            DelegateConfig::default(),
        )?;
        Ok(())
    }
}
