use crate::board::checkers::Checkers;
use crate::board::Board;
use crate::constant::error::TakeError;
use crate::constant::player::Side;
use crate::constant::result::CheckerAvailability;
use crate::stage::checker_taken::CheckerTaken;
use crate::stage::Stage;
use crate::types::checker_move::CheckerMove;
use crate::types::dice_pair::DicePair;
use crate::types::pip::Pip;

pub struct DicesThrown {
    board: Board,
    done_moves: Vec<CheckerMove>,
    active_side: Side,
    dice_pair: DicePair,
    focused_pip: Pip,
}

impl Stage for DicesThrown {
    fn white_checkers(&self) -> Checkers { self.board.white_checkers }
    fn black_checkers(&self) -> Checkers { self.board.black_checkers }
    fn active_side(&self) -> Option<Side> { Some(self.active_side) }
    fn dice_pair(&self) -> Option<DicePair> { Some(self.dice_pair) }
    fn taken_checker_pip(&self) -> Option<Pip> { None }
    fn focused_pip(&self) -> Option<Pip> { Some(self.focused_pip) }
}

impl DicesThrown {
    pub fn new(board: Board,
               done_moves: Vec<CheckerMove>,
               active_side: Side,
               dice_pair: DicePair) -> Self {

        Self {
            board,
            active_side,
            dice_pair,
            done_moves,
            focused_pip: Pip::new(0)
        }
    }

    pub fn focus_pip(&mut self, pip: Pip) {
        self.focused_pip = pip;
    }

    pub fn take_checker(self) -> Result<CheckerTaken, TakeError> {
        match self.board.get_checker_availability(self.active_side, self.focused_pip) {
            CheckerAvailability::NoCheckerFound =>
                return Err(TakeError::NotEnoughCheckers),
            CheckerAvailability::ReferringToOpponentPip =>
                return Err(TakeError::TakingOpponentPip),
            _ => {}
        };

        Ok(
            CheckerTaken::new(self.board, self.done_moves, self.focused_pip, self.active_side, self.dice_pair)
        )
    }
}
