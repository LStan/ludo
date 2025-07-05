use anchor_lang::prelude::*;

use crate::{Colors, Game, GameState, LudoError, GAME};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(init, payer = player, space = 8 + Game::INIT_SPACE, seeds = [GAME, seed.to_le_bytes().as_ref()], bump)]
    pub game: Account<'info, Game>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateGame<'info> {
    pub fn create_game(
        &mut self,
        seed: u64,
        num_players: u8,
        color: Colors,
        bumps: &CreateGameBumps,
    ) -> Result<()> {
        require!(
            num_players == 2 || num_players == 3 || num_players == 4,
            LudoError::InvalidNumPlayers
        );
        self.game.set_inner(Game {
            seed,
            bump: bumps.game,
            num_players,
            cur_player: 1,
            token_positions: [[-1; 4]; 4],
            game_state: GameState::NotStarted,
            current_roll: 0,
            six_count: 0,
            players: [Pubkey::default(); 4],
            winner: Pubkey::default(),
        });
        self.game.players[color as usize] = self.player.key();
        Ok(())
    }
}
