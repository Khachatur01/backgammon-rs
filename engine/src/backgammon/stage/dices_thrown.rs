use crate::backgammon::stage::checker_taken::CheckerTaken;
use crate::board::Board;
use crate::constant::error::TakeError;
use crate::constant::player::Side;
use crate::constant::result::CheckerAvailability;
use crate::types::dices::DicePair;
use crate::types::pip::Pip;
use crate::types::r#move::Move;

pub struct DicesThrown {
    board: Board,
    moves_done: Vec<Move>,
    active_side: Side,
    dice_pair: DicePair,
}

impl DicesThrown {
    pub fn new(board: Board, moves_done: Vec<Move>, active_side: Side, dice_pair: DicePair) -> Self {
        Self {
            board, active_side, dice_pair, moves_done
        }
    }

    pub fn take_checker(self, from_pip: Pip) -> Result<CheckerTaken, TakeError> {
        /* TODO: check if checker can be taken */

        match self.board.get_checker_availability(self.active_side, Pip::from(from_pip)) {
            CheckerAvailability::NoCheckerFound =>
                return Err(TakeError::NotEnoughCheckers),
            CheckerAvailability::ReferringToOpponentPip =>
                return Err(TakeError::TakingOpponentPip),
            _ => {}
        };

        Ok(
            CheckerTaken::new(self.board, self.moves_done, from_pip, self.active_side, self.dice_pair)
        )
    }
}
