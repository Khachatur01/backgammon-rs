pub enum TakeError {
    NotEnoughCheckers,
    TakingOpponentPip
}

pub enum MoveError {
    BlockingOpponent,
    PlaceIsTakenByOpponent
}

pub enum BearOffError {
    NotAllCheckersAreInHome
}

pub enum CommitError {
    NotAllDicesPlayed
}

pub enum ThrowDicesError {
    DicesAlreadyThrown,
    GameDoesntNotStartedYet
}

pub enum SwitchSideError {
    ActiveSideDoesntFinishPlaying
}
