use anchor_lang::prelude::*;

#[constant]
pub const GAME: &[u8] = b"ludo_game";

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum Colors {
    Red = 0,
    Green = 1,
    Yellow = 2,
    Blue = 3,
}
