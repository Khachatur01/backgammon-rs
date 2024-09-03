use crate::board::Board;
use crate::constant::player::Side;
use crate::stage::dices_thrown::DicesThrown;
use crate::types::dice_pair::DicePair;
use crate::types::checker_move::CheckerMove;

pub struct SideSwitched {
    board: Board,
    moves_done: Vec<CheckerMove>,
    active_side: Side,
    dice_pair: DicePair,
}

impl SideSwitched {
    pub fn new(board: Board,
               moves_done: Vec<CheckerMove>,
               active_side: Side,
               dice_pair: DicePair) -> Self {
        Self {
            board, moves_done, active_side, dice_pair
        }
    }

    pub fn throw_dices(self) -> DicesThrown {
        /* TODO: generate new dice pair */

        DicesThrown::new(self.board, self.moves_done, self.active_side, self.dice_pair)
    }
}
