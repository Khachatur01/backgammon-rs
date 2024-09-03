use crate::backgammon::stage::dices_thrown::DicesThrown;
use crate::board::Board;
use crate::constant::error::CommitError;
use crate::constant::player::Side;
use crate::stage::moves_commited::MovesCommited;
use crate::stage::side_switched::SideSwitched;
use crate::stage::win::Win;
use crate::types::dice_pair::DicePair;
use crate::types::checker_move::CheckerMove;

pub struct OutOfMoves {
    board: Board,
    moves_done: Vec<CheckerMove>,
    active_side: Side,
    dice_pair: DicePair,
}

impl OutOfMoves {
    pub fn new(board: Board, moves_done: Vec<CheckerMove>, active_side: Side, dice_pair: DicePair) -> Self {
        Self {
            board, moves_done, active_side, dice_pair
        }
    }

    pub fn commit_moves(self) -> Result<MovesCommited, CommitError> {
        let next_stage: MovesCommited = match true {
            true => {
                MovesCommited::Win(
                    Win {}
                )
            }
            false => {
                /* Switch active side */
                let new_active_side: Side = match self.active_side {
                    Side::White => Side::Black,
                    Side::Black => Side::White,
                };

                MovesCommited::SideSwitched(
                    SideSwitched::new(self.board, self.moves_done, new_active_side, self.dice_pair)
                )
            }
        };

        Ok(next_stage)
    }

    pub fn cancel_moves(self) -> DicesThrown {
        /* TODO: undo all done moves */

        DicesThrown::new(self.board, self.moves_done, self.active_side, self.dice_pair)
    }
}
