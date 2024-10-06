use crate::board::checkers::Checkers;
use crate::board::Board;
use crate::constant::player::Side;
use crate::stage::Stage;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;

pub struct Win {
    board: Board,
    active_side: Side,
    dice_pair: DicePair,
}

impl Stage for Win {
    fn white_checkers(&self) -> Checkers { self.board.white_checkers }
    fn black_checkers(&self) -> Checkers { self.board.black_checkers }
    fn active_side(&self) -> Option<Side> { Some(self.active_side) }
    fn dice_pair(&self) -> Option<DicePair> { Some(self.dice_pair) }
    fn taken_checker_pip(&self) -> Option<Pip> { None }
    fn focused_pip(&self) -> Option<Pip> { None }
}

impl Win {
    pub fn new(board: Board,
               active_side: Side,
               dice_pair: DicePair) -> Self {

        Self {
            board,
            active_side,
            dice_pair
        }
    }
}
