use std::cell::RefCell;
use std::rc::Rc;
use crate::backgammon::stage::dices_thrown::DicesThrown;
use crate::Backgammon;

pub struct MovesCommited {
    backgammon: Rc<RefCell<Backgammon>>
}

impl MovesCommited {
    pub fn new(backgammon: Rc<RefCell<Backgammon>>) -> Self {
        Self { backgammon }
    }

    pub fn throw_dices(&mut self) -> DicesThrown {
        let backgammon = self.backgammon.borrow_mut();

        DicesThrown::new(self.backgammon.clone())
    }
}
