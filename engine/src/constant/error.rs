use std::fmt::Display;

#[derive(Debug)]
pub enum TakeError<SELF> {
    NotEnoughCheckers(SELF),
    TakingOpponentPip(SELF)
}

#[derive(Debug)]
pub enum MoveError<SELF> {
    BlockingOpponent(SELF),
    PipIsOccupiedByOpponent(SELF),
    InconsistentWithDices(SELF)
}

#[derive(Debug)]
pub enum BearOffError<SELF> {
    NotAllCheckersAreInHome(SELF),
    InconsistentWithDices(SELF)
}

#[derive(Debug)]
pub enum CommitError<SELF> {
    NotAllDicesPlayed(SELF)
}

#[derive(Debug)]
pub enum ThrowDicesError<SELF> {
    DicesAlreadyThrown(SELF),
    GameDoesntNotStartedYet(SELF)
}

#[derive(Debug)]
pub enum SwitchSideError<SELF> {
    ActiveSideDoesntFinishPlaying(SELF)
}
