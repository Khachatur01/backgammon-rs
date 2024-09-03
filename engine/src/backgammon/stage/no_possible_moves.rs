use crate::board::checkers::Checkers;
use crate::board::Board;
use crate::constant::player::Side;
use crate::stage::side_switched::SideSwitched;
use crate::stage::Stage;
use crate::types::checker_move::CheckerMove;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;

pub struct NoPossibleMoves {
    board: Board,
    done_moves: Vec<CheckerMove>,
    active_side: Side,
    dice_pair: DicePair,
}

impl Stage for NoPossibleMoves {
    fn white_checkers(&self) -> &Checkers { &self.board.white_checkers }
    fn black_checkers(&self) -> &Checkers { &self.board.black_checkers }
    fn active_side(&self) -> Option<Side> { Some(self.active_side) }
    fn dice_pair(&self) -> Option<DicePair> { Some(self.dice_pair) }
    fn taken_checker_pip(&self) -> Option<Pip> { None }
}

impl NoPossibleMoves {
    pub fn new(board: Board,
               done_moves: Vec<CheckerMove>,
               active_side: Side,
               dice_pair: DicePair) -> Self {

        Self {
            board,
            done_moves,
            active_side,
            dice_pair
        }
    }

    pub fn switch_side(self) -> SideSwitched {
        let new_active_side: Side = match self.active_side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };

        SideSwitched::new(self.board, self.done_moves, new_active_side, self.dice_pair)
    }
}