use anchor_lang::prelude::*;

use crate::{Colors, Game, GameState, LudoError, GAME};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}

impl<'info> JoinGame<'info> {
    pub fn join_game(&mut self, color: Colors) -> Result<()> {
        let game = &mut self.game;

        require!(
            game.game_state == GameState::NotStarted,
            LudoError::GameAlreadyStarted
        );

        require!(
            game.cur_player + 1 < game.num_players,
            LudoError::NeedToRunJoinAndStart
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

        game.cur_player += 1;

        Ok(())
    }
}
