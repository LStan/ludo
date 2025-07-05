use anchor_lang::prelude::*;

use crate::{Colors, Game, GameState, GAME};

#[derive(Accounts)]
pub struct Debug<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}

impl<'info> Debug<'info> {
    pub fn join_and_start_game_debug(&mut self, color: Colors, start_player: Colors) -> Result<()> {
        let game = &mut self.game;
        game.players[color as usize] = self.player.key();
        game.cur_player = start_player as u8;
        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn roll_dice_debug(&mut self, roll: u8) -> Result<()> {
        let game = &mut self.game;
        game.current_roll = roll;
        game.game_state = GameState::Move;
        Ok(())
    }

    pub fn next_turn_debug(&mut self) -> Result<()> {
        let game = &mut self.game;
        game.six_count = 0;
        game.next_player();
        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn move_token_debug(&mut self, color: Colors, token_num: u8, position: i8) -> Result<()> {
        let game = &mut self.game;
        game.token_positions[color as usize][token_num as usize] = position;
        Ok(())
    }
}
