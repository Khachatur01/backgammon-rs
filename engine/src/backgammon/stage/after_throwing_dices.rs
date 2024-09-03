use crate::stage::dices_thrown::DicesThrown;
use crate::stage::no_possible_moves::NoPossibleMoves;

pub enum AfterThrowingDices {
    NoPossibleMoves(NoPossibleMoves),
    DicesThrown(DicesThrown)
}
