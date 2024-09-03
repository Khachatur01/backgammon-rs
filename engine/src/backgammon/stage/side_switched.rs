use crate::board::Board;
use crate::board::checkers::Checkers;
use crate::constant::player::Side;
use crate::stage::dices_thrown::DicesThrown;
use crate::stage::Stage;
use crate::types::checker_move::CheckerMove;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;

pub struct SideSwitched {
    board: Board,
    done_moves: Vec<CheckerMove>,
    active_side: Side,
    dice_pair: DicePair,
}

impl Stage for SideSwitched {
    fn white_checkers(&self) -> &Checkers { &self.board.white_checkers }
    fn black_checkers(&self) -> &Checkers { &self.board.black_checkers }
    fn active_side(&self) -> Option<Side> { Some(self.active_side) }
    fn dice_pair(&self) -> Option<DicePair> { Some(Self.dice_pair) }
    fn taken_checker_pip(&self) -> Option<Pip> { None }
}

impl SideSwitched {
    pub fn new(board: Board,
               done_moves: Vec<CheckerMove>,
               active_side: Side,
               dice_pair: DicePair) -> Self {
        Self {
            board, done_moves, active_side, dice_pair
        }
    }

    pub fn throw_dices(self) -> DicesThrown {
        /* TODO: generate new dice pair */

        DicesThrown::new(self.board, self.done_moves, self.active_side, self.dice_pair)
    }
}
