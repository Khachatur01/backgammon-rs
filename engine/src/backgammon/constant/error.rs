pub enum TakeError {
    NotEnoughCheckers,
    TakingOpponentPip
}

pub enum MoveError {
    BlockingOpponent,
    NoCheckerTaken
}

pub enum BearOffError {
    NotAllCheckersAreInHome,
    NoCheckerTaken
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
