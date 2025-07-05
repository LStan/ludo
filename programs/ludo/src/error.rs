use anchor_lang::prelude::*;

#[error_code]
pub enum LudoError {
    InvalidNumPlayers,
    AnotherPlayerAlreadyJoined,
    GameAlreadyStarted,
    WrongPlayer,
    NeedToRunJoinAndStart,
    NeedToRunJoin,
    PlayerAlreadyJoined,
    ColorAlreadyTaken,
    WrongGameState,
    WrongMove,
    GameNotFinished,
}
