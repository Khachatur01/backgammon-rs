use crate::backgammon::stage::checker_taken::CheckerTaken;
use crate::backgammon::stage::dices_thrown::DicesThrown;
use crate::backgammon::stage::moves_commited::MovesCommited;
use crate::backgammon::stage::out_of_moves::OutOfMoves;
use crate::backgammon::stage::start::Start;

pub mod start;
pub mod dices_thrown;
pub mod checker_taken;
pub mod out_of_moves;
pub mod moves_commited;

pub enum Stage {
    Start(Start),
    DicesThrown(DicesThrown),
    CheckerTaken(CheckerTaken),
    OutOfMoves(OutOfMoves),
    MovesCommited(MovesCommited)
}
