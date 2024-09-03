use crate::backgammon::stage::dices_thrown::DicesThrown;
use crate::board::Board;
use crate::constant::player::Side;
use crate::types::dices::DicePair;
use crate::types::r#move::Move;
use rand::Rng;

pub struct Start {
    board: Board,
    moves_done: Vec<Move>,
}

impl Start {
    pub fn new(board: Board, moves_done: Vec<Move>) -> Self {
        Self {
            board, moves_done
        }
    }

    pub fn throw_dices(self) -> DicesThrown {
        /* generate random dices until dices are equal */
        let dice_pair: DicePair = loop {
            let first_dice: u8 = rand::thread_rng().gen_range(1..=6);
            let second_dice: u8 = rand::thread_rng().gen_range(1..=6);

            if first_dice != second_dice {
                break DicePair::new(first_dice, second_dice);
            }
        };

        let active_side: Side =
            if dice_pair.first() > dice_pair.second() {
                Side::White
            } else {
                Side::Black
            };

        DicesThrown::new(self.board, self.moves_done, active_side, dice_pair)
    }
}
