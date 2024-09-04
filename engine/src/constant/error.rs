#[derive(Debug)]
pub enum TakeError {
    NotEnoughCheckers,
    TakingOpponentPip
}

#[derive(Debug)]
pub enum MoveError {
    BlockingOpponent,
    PipIsOccupiedByOpponent,
    InconsistentWithDices
}

#[derive(Debug)]
pub enum BearOffError {
    NotAllCheckersAreInHome,
    InconsistentWithDices
}

#[derive(Debug)]
pub enum CommitError {
    NotAllDicesPlayed
}

#[derive(Debug)]
pub enum ThrowDicesError {
    DicesAlreadyThrown,
    GameDoesntNotStartedYet
}

#[derive(Debug)]
pub enum SwitchSideError {
    ActiveSideDoesntFinishPlaying
}
