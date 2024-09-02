use std::arch::x86_64::_bextr2_u32;
use crate::backgammon;
use crate::backgammon::board::Board;
use crate::backgammon::constant::player::Side;
use crate::backgammon::stage::start::Start;
use crate::backgammon::types::dices::DicePair;
use crate::types::from_pip::FromPip;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use crate::types::r#move::Move;

pub mod constant;
pub mod board;
pub mod types;
pub mod stage;

pub type Result<E> = std::result::Result<(), E>;

/**
    If we have active side, then game is already started.
    If we have winner side, then game is already over.
*/
pub struct Backgammon {
    pub active_side: Option<Side>,
    pub winner_side: Option<Side>,

    pub taken_checker: Option<FromPip>,
    pub dice_pair: Option<DicePair>,
    pub moves_done: Vec<Move>,
    pub board: Board,
}

impl Backgammon {
    pub fn new() -> Start {
        let backgammon: Backgammon = Self {
            active_side: None,
            winner_side: None,

            taken_checker: None,
            dice_pair: None,
            moves_done: vec![],
            board: Board::new(),
        };

        Start::new(Rc::new(RefCell::new(backgammon)))
    }

    pub fn throw_first_dices(&mut self) {

    }
}
