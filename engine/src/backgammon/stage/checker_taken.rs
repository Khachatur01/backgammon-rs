use crate::backgammon::stage::dices_thrown::DicesThrown;
use crate::backgammon::stage::out_of_moves::OutOfMoves;
use crate::board::Board;
use crate::constant::error::MoveError;
use crate::constant::player::Side;
use crate::constant::result::CheckerAvailability;
use crate::stage::checker_moved::CheckerMoved;
use crate::types::dices::DicePair;
use crate::types::pip::Pip;
use crate::types::r#move::Move;
use crate::types::r#move::Move::Step;

pub struct CheckerTaken {
    board: Board,
    moves_done: Vec<Move>,
    from_pip: Pip,
    active_side: Side,
    dice_pair: DicePair,
}

impl CheckerTaken {
    pub fn new(board: Board,
               moves_done: Vec<Move>,
               from_pip: Pip,
               active_side: Side,
               dice_pair: DicePair) -> Self {
        Self {
            board, moves_done, from_pip, active_side, dice_pair
        }
    }

    pub fn move_checker(mut self, to_pip: Pip) -> Result<CheckerMoved, MoveError> {
        let form_pip: Pip = self.from_pip;

        match self.board.get_checker_availability(self.active_side, Pip::from(to_pip)) {
            CheckerAvailability::ReferringToOpponentPip =>
                return Err(MoveError::PipIsOccupiedByOpponent),
            _ => {}
        };

        /* TODO: check move validity */
        self.board.move_checker(self.active_side, Step(form_pip, to_pip));

        let next_stage: CheckerMoved = match true {
            true => {
                CheckerMoved::DicesThrown(
                    DicesThrown::new(self.board, self.moves_done, self.active_side, self.dice_pair)
                )
            }
            false => {
                CheckerMoved::OutOfMoves(
                    OutOfMoves::new(self.board, self.moves_done, self.active_side, self.dice_pair)
                )
            }
        };

        Ok(next_stage)
    }

    pub fn cancel(self) -> DicesThrown {
        DicesThrown::new(self.board, self.moves_done, self.active_side, self.dice_pair)
    }
}
