use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;
use crate::Backgammon;
use crate::backgammon::stage::dices_thrown::DicesThrown;
use crate::constant::player::Side;
use crate::types::dices::DicePair;

pub struct Start {
    backgammon: Rc<RefCell<Backgammon>>
}

impl Start {
    pub fn new(backgammon: Rc<RefCell<Backgammon>>) -> Self {
        Self { backgammon }
    }

    pub fn throw_dices(&mut self) -> DicesThrown {
        let mut backgammon = self.backgammon.borrow_mut();

        /* generate random dices until dices are equal */
        let dice_pair: DicePair = loop {
            let first_dice: u8 = rand::thread_rng().gen_range(1..=6);
            let second_dice: u8 = rand::thread_rng().gen_range(1..=6);

            if first_dice != second_dice {
                break DicePair::new(first_dice, second_dice);
            }
        };

        backgammon.active_side =
            if dice_pair.first() > dice_pair.second() {
                Some(Side::White)
            } else {
                Some(Side::Black)
            };

        backgammon.dice_pair = Some(dice_pair);

        DicesThrown::new(self.backgammon.clone())
    }
}
