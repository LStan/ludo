use anchor_lang::prelude::*;

use crate::{Game, GameState, LudoError, GAME};

pub const SAFE_POSITIONS: [i8; 8] = [0, 13, 26, 39, 8, 21, 34, 47];

#[derive(Accounts)]
pub struct Move<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [GAME, game.seed.to_le_bytes().as_ref()], bump = game.bump)]
    pub game: Account<'info, Game>,
}

impl<'info> Move<'info> {
    pub fn token_into_play(&mut self, token_num: u8) -> Result<()> {
        let game = &mut self.game;
        require!(
            game.game_state == GameState::Move,
            LudoError::WrongGameState
        );

        let cur_player = game.cur_player;

        require!(
            game.cur_player_key() == self.player.key(),
            LudoError::WrongPlayer
        );

        require!(game.current_roll == 6, LudoError::WrongMove);

        // this should never happen because the turn should be skipped in callback_roll_dice
        require!(game.six_count < 2, LudoError::WrongMove);

        require!(
            game.token_positions[cur_player as usize][token_num as usize] == -1,
            LudoError::WrongMove
        );

        game.token_positions[cur_player as usize][token_num as usize] = 0;

        game.game_state = GameState::RollDice;
        Ok(())
    }

    pub fn make_move(&mut self, token_num: u8) -> Result<()> {
        let game = &mut self.game;
        require!(
            game.game_state == GameState::Move,
            LudoError::WrongGameState
        );

        let cur_player = game.cur_player;

        require!(
            game.cur_player_key() == self.player.key(),
            LudoError::WrongPlayer
        );

        // this should never happen because the turn should be skipped in callback_roll_dice
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
            game.winner = self.player.key();
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
}
