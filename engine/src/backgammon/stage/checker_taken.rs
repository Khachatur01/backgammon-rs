use std::cell::RefCell;
use std::rc::Rc;
use crate::backgammon::stage::dices_thrown::DicesThrown;
use crate::backgammon::stage::out_of_moves::OutOfMoves;
use crate::Backgammon;
use crate::constant::error::{MoveError, TakeError};
use crate::constant::player::Side;
use crate::constant::result::CheckerAvailability;
use crate::types::from_pip::FromPip;
use crate::types::pip::Pip;
use crate::types::r#move::Move::Step;
use crate::types::to_pip::ToPip;

pub enum CheckerMoved {
    DicesThrown(DicesThrown),
    OutOfMoves(OutOfMoves),
}

pub struct CheckerTaken {
    backgammon: Rc<RefCell<Backgammon>>
}

impl CheckerTaken {
    pub fn new(backgammon: Rc<RefCell<Backgammon>>) -> Self {
        Self {
            backgammon
        }
    }

    pub fn move_checker(&mut self, to_pip: ToPip) -> Result<CheckerMoved, MoveError> {
        let mut backgammon = self.backgammon.borrow_mut();

        let active_side: Side = backgammon.active_side.expect(
            format!("Can't take checker {from_pip}. No active side").as_str()
        );

        let taken_checker: FromPip = backgammon.taken_checker.take().unwrap();

        match backgammon.board.get_checker_availability(active_side, Pip::from(to_pip)) {
            CheckerAvailability::ReferringToOpponentPip =>
                return Err(MoveError::PipIsOccupiedByOpponent),
            _ => {}
        };

        /* TODO: check move validity */
        backgammon.board.move_checker(backgammon.active_side.unwrap(), Step(taken_checker, to_pip));

        let next_stage: CheckerMoved = match true {
            true => {
                CheckerMoved::DicesThrown(
                    DicesThrown::new(self.backgammon.clone())
                )
            }
            false => {
                CheckerMoved::OutOfMoves(
                    OutOfMoves::new(self.backgammon.clone())
                )
            }
        };

        Ok(next_stage)
    }

    pub fn cancel(&mut self) -> DicesThrown {
        self.backgammon.borrow_mut().taken_checker = None;

        DicesThrown::new(self.backgammon.clone())
    }
}
