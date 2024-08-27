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
