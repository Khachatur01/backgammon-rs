use crate::backgammon::types::from_pip::FromPip;
use crate::backgammon::types::to_pip::ToPip;

pub enum Move {
    Step(FromPip, ToPip),
    BearOff(FromPip)
}