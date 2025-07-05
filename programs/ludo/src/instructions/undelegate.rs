use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

use crate::{Game, GAME};

#[commit]
#[derive(Accounts)]
pub struct Undelegate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}

impl<'info> Undelegate<'info> {
    pub fn undelegate(&mut self) -> Result<()> {
        // require!(
        //     ctx.accounts.game.game_state == GameState::Finished,
        //     LudoError::GameNotFinished
        // );
        commit_and_undelegate_accounts(
            &self.payer,
            vec![&self.game.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;
        Ok(())
    }
}
