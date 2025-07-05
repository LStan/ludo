use anchor_lang::prelude::*;

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
            if self.cur_player_key() != Pubkey::default() {
                break;
            }
        }
    }

    #[inline(always)]
    pub fn cur_player_key(&self) -> Pubkey {
        self.players[self.cur_player as usize]
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
