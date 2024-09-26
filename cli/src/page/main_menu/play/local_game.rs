use std::mem::swap;
use std::rc::Rc;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{DummyView, LinearLayout, TextView};
use cursive::Cursive;
use engine::constant::player::Side;
use engine::stage::PossibleStage;
use engine::stage::start::Start;
use engine::start_game;
use crate::custom_view::stage_view::StageView;
use crate::stage_theme::half_width::HalfWidth;
use crate::stage_theme::height::Height;
use crate::stage_theme::percent::Percent;
use crate::stage_theme::StageTheme;

pub fn open_local_game_page(cursive: &mut Cursive) {
    cursive.pop_layer();

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

    let mut current_stage: PossibleStage = PossibleStage::Start(start_game());

    let board_layout =
        LinearLayout::vertical()
            .with_name("board");

    let information_layout =
        LinearLayout::vertical()
            .child(TextView::new("Move is for -> Black"));

    let vertical_layout =
        LinearLayout::horizontal()
            .child(board_layout)
            .child(DummyView)
            .child(information_layout)
            .full_screen();

    cursive.add_layer(vertical_layout);


    cursive.call_on_name("board", |view: &mut LinearLayout| {
        view.remove_child(0);

        match current_stage {
            PossibleStage::Start(start) => {
                let start_stage_view: StageView = StageView::from(&start, stage_theme, Side::White);
                view.add_child(start_stage_view);

                Rc::new(PossibleStage::DicesThrown(start.throw_dices()))
            }
            PossibleStage::DicesThrown(dices_thrown) => {
                Rc::new(PossibleStage::DicesThrown(dices_thrown))
            }
            PossibleStage::AfterThrowingDices(after_throwing_dices) => {
                Rc::new(PossibleStage::AfterThrowingDices(after_throwing_dices))
            }
            PossibleStage::CheckerTaken(checker_taken) => {
                Rc::new(PossibleStage::CheckerTaken(checker_taken))
            }
            PossibleStage::CheckerMoved(checker_moved) => {
                Rc::new(PossibleStage::CheckerMoved(checker_moved))
            }
            PossibleStage::NoPossibleMoves(no_possible_moves) => {
                Rc::new(PossibleStage::NoPossibleMoves(no_possible_moves))
            }
            PossibleStage::OutOfMoves(out_of_moves) => {
                Rc::new(PossibleStage::OutOfMoves(out_of_moves))
            }
            PossibleStage::MovesCommited(moves_commited) => {
                Rc::new(PossibleStage::MovesCommited(moves_commited))
            }
            PossibleStage::SideSwitched(side_switched) => {
                Rc::new(PossibleStage::SideSwitched(side_switched))
            }
            PossibleStage::Win(win) => {
                Rc::new(PossibleStage::Win(win))
            }
        }
    });
}
