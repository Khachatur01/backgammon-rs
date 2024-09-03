use crate::stage::dices_thrown::DicesThrown;
use crate::stage::out_of_moves::OutOfMoves;

pub enum CheckerMoved {
    DicesThrown(DicesThrown),
    OutOfMoves(OutOfMoves),
}
