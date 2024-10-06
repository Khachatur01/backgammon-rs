use crate::board::checkers::Checkers;
use crate::constant::player::Side;
use crate::stage::after_throwing_dices::AfterThrowingDices;
use crate::stage::checker_moved::CheckerMoved;
use crate::stage::checker_taken::CheckerTaken;
use crate::stage::dices_thrown::DicesThrown;
use crate::stage::moves_commited::MovesCommited;
use crate::stage::no_possible_moves::NoPossibleMoves;
use crate::stage::out_of_moves::OutOfMoves;
use crate::stage::side_switched::SideSwitched;
use crate::stage::started::Started;
use crate::stage::win::Win;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;

pub mod started;
pub mod dices_thrown;
pub mod checker_taken;
pub mod out_of_moves;
pub mod checker_moved;
pub mod win;
pub mod side_switched;
pub mod moves_commited;
pub mod no_possible_moves;
pub mod after_throwing_dices;

pub trait Stage {
    fn white_checkers(&self) -> Checkers;
    fn black_checkers(&self) -> Checkers;
    fn active_side(&self) -> Option<Side>;
    fn dice_pair(&self) -> Option<DicePair>;
    fn taken_checker_pip(&self) -> Option<Pip>;
    fn focused_pip(&self) -> Option<Pip>;
}

pub enum PossibleStage {
    Started(Started),
    DicesThrown(DicesThrown),
    AfterThrowingDices(AfterThrowingDices),
    CheckerTaken(CheckerTaken),
    CheckerMoved(CheckerMoved),
    NoPossibleMoves(NoPossibleMoves),
    OutOfMoves(OutOfMoves),
    MovesCommited(MovesCommited),
    SideSwitched(SideSwitched),
    Win(Win)
}
