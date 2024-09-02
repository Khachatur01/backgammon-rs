use std::cell::RefCell;
use std::rc::Rc;
use crate::Backgammon;
use crate::backgammon::stage::checker_taken::CheckerTaken;
use crate::constant::error::TakeError;
use crate::constant::player::Side;
use crate::constant::result::CheckerAvailability;
use crate::types::from_pip::FromPip;
use crate::types::pip::Pip;

pub struct DicesThrown {
    backgammon: Rc<RefCell<Backgammon>>
}

impl DicesThrown {
    pub fn new(backgammon: Rc<RefCell<Backgammon>>) -> Self {
        Self { backgammon }
    }

    pub fn take_checker(&mut self, from_pip: FromPip) -> Result<CheckerTaken, TakeError> {
        let mut backgammon = self.backgammon.borrow_mut();

        let active_side: Side = backgammon.active_side.expect(
            format!("Can't take checker {from_pip}. No active side").as_str()
        );

        match backgammon.board.get_checker_availability(active_side, Pip::from(from_pip)) {
            CheckerAvailability::NoCheckerFound =>
                return Err(TakeError::NotEnoughCheckers),
            CheckerAvailability::ReferringToOpponentPip =>
                return Err(TakeError::TakingOpponentPip),
            _ => {}
        };

        backgammon.taken_checker = Some(from_pip);

        Ok(
            CheckerTaken::new(self.backgammon.clone())
        )
    }
}
