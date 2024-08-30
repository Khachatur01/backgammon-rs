use crate::constant::player::Side;
use crate::types::dices::DicePair;
use crate::types::from_pip::FromPip;
use crate::types::to_pip::ToPip;

pub enum Action {
    ThrowFirstDices(DicePair),
    TakeChecker(FromPip),
    MoveChecker(ToPip),
    BearOffChecker(ToPip),
    CommitMoves,
    ThrowDices(DicePair),
    Win(Side)
}
