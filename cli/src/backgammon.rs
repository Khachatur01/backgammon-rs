use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use cursive::event;
use engine::constant::player::Side;
use engine::stage::PossibleStage;
use engine::start_game;
use crate::custom_view::stage_view::StageView;
use crate::stage_theme::half_width::HalfWidth;
use crate::stage_theme::height::Height;
use crate::stage_theme::percent::Percent;
use crate::stage_theme::StageTheme;

pub struct Backgammon {
    current_stage: PossibleStage,
    view_sender: Sender<StageView>,
}

impl Backgammon {
    pub fn new() -> (Self, Sender<event::Key>, Receiver<StageView>) {
        let (view_sender, view_receiver) = mpsc::channel();
        let (event_sender, event_receiver) = mpsc::channel();

        let this: Self = Self {
            current_stage: PossibleStage::NotStarted,
            view_sender,
        };

        thread::spawn(move || {
            while let Ok(key) = event_receiver.recv() {
                println!("Key received {:?}", key);
            }
        });

        (this, event_sender, view_receiver)
    }

    pub fn start(&self) {
        let stage_theme: StageTheme = StageTheme {
            numbers: ['⑴', '⑵', '⑶', '⑷', '⑸', '⑹', '⑺', '⑻', '⑼', '⑽', '⑾', '⑿', '⒀', '⒁', '⒂'],
            dices: ['⚀', '⚁', '⚂', '⚃', '⚄', '⚅'],
            board_border: '█',
            space: ' ',
            pips_separator: '|',
            white_checker: '⛂',
            black_checker: '⛀',
            possible_move: '🞙',
            up: '⮝',
            down: '⮟',
            right: '⮞',
            half_width: HalfWidth::new(18),
            bore_off_column_width: 1,
            height: Height::new(20),
            peaces_cut_off_height_percent: Percent::new(40),
        };

        let start_stage_view: StageView = StageView::from(&start_game(), stage_theme, Side::White);
        self.view_sender.send(start_stage_view).unwrap()
    }
}
