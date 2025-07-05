use anchor_lang::prelude::*;

use crate::{Colors, Game, GameState, LudoError, GAME};

#[derive(Accounts)]
pub struct CancelGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump, close = player)]
    pub game: Account<'info, Game>,
}

impl<'info> CancelGame<'info> {
    pub fn cancel_game(&mut self, color: Colors) -> Result<()> {
        let game = &mut self.game;

        require!(
            game.game_state == GameState::NotStarted,
            LudoError::GameAlreadyStarted
        );

        require!(game.cur_player == 1, LudoError::AnotherPlayerAlreadyJoined);

        require!(
            game.players[color as usize] == self.player.key(),
            LudoError::WrongPlayer
        );
        Ok(())
    }
}
