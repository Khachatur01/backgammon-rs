use std::cell::RefCell;
use std::rc::Rc;
use crate::backgammon::stage::dices_thrown::DicesThrown;
use crate::backgammon::stage::moves_commited::MovesCommited;
use crate::Backgammon;
use crate::constant::error::CommitError;
use crate::constant::player::Side;

pub struct OutOfMoves {
    backgammon: Rc<RefCell<Backgammon>>
}

impl OutOfMoves {
    pub fn new(backgammon: Rc<RefCell<Backgammon>>) -> Self {
        Self { backgammon }
    }


    pub fn commit_moves(&mut self) -> Result<MovesCommited, CommitError> {
        let mut backgammon = self.backgammon.borrow_mut();
        let active_side: Side = backgammon.active_side.expect("Can't commit moves. No active Side");

        /* Switch active side */
        backgammon.active_side = match active_side {
            Side::White => Some(Side::Black),
            Side::Black => Some(Side::White),
        };

        backgammon.dice_pair = None;

        Ok(
            MovesCommited::new(self.backgammon.clone())
        )
    }

    pub fn cancel_moves(&mut self) -> DicesThrown {
        let backgammon = self.backgammon.borrow_mut();

        /* TODO: undo all done moves */

        backgammon.moves_done = vec![];

        DicesThrown::new(self.backgammon.clone())
    }
}
