use crate::custom_view::stage_view::StageView;
use crate::stage_theme::half_width::HalfWidth;
use crate::stage_theme::height::Height;
use crate::stage_theme::percent::Percent;
use crate::stage_theme::StageTheme;
use cursive::event::{Event, Key};
use engine::constant::player::Side;
use engine::stage::{PossibleStage, Stage};
use engine::{stage, start_game};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use engine::types::pip::Pip;

pub fn new() -> (Sender<Event>, Receiver<StageView>) {
    let (view_sender, view_receiver) = mpsc::channel();
    let (event_sender, event_receiver) = mpsc::channel();

    let stage_theme: StageTheme = StageTheme {
        numbers: ['⑴', '⑵', '⑶', '⑷', '⑸', '⑹', '⑺', '⑻', '⑼', '⑽', '⑾', '⑿', '⒀', '⒁', '⒂'],
        dices: ['⚀', '⚁', '⚂', '⚃', '⚄', '⚅'],
        board_border: '█',
        space: ' ',
        pips_separator: '|',
        white_checker: '⛂',
        black_checker: '⛀',
        possible_move: '🞙',
        focused_pip: '⯌',
        up: '🖢',
        down: '🖣',
        right: '🖝',
        left: '🖜',
        half_width: HalfWidth::new(18),
        bore_off_column_width: 1,
        height: Height::new(20),
        peaces_cut_off_height_percent: Percent::new(40),
    };

    thread::spawn(move || {
        start(stage_theme, view_sender, event_receiver);
    });

    (event_sender, view_receiver)
}

fn start(stage_theme: StageTheme, view_sender: Sender<StageView>, event_receiver: Receiver<Event>) {
    let started_stage: stage::started::Started = start_game();
    let start_stage_view: StageView = StageView::from(&started_stage, stage_theme, Side::White);

    view_sender.clone().send(start_stage_view).unwrap_or(());

    let mut current_stage = PossibleStage::Started(started_stage);

    while let Ok(event) = event_receiver.recv() {
        match event {
            Event::Key(Key::Enter) => {
                current_stage = match current_stage {
                    PossibleStage::Started(started_stage) => {
                        let dices_thrown_stage: stage::dices_thrown::DicesThrown = started_stage.throw_dices();
                        let dices_thrown_stage_view: StageView = StageView::from(&dices_thrown_stage, stage_theme, Side::White);

                        view_sender.clone().send(dices_thrown_stage_view).unwrap_or(());
                        PossibleStage::DicesThrown(dices_thrown_stage)
                    }
                    PossibleStage::DicesThrown(dices_thrown_stage) => {
                        PossibleStage::DicesThrown(dices_thrown_stage)
                    }
                    PossibleStage::AfterThrowingDices(after_throwing_dices) => {
                        PossibleStage::AfterThrowingDices(after_throwing_dices)
                    }
                    PossibleStage::CheckerTaken(checker_taken) => {
                        PossibleStage::CheckerTaken(checker_taken)
                    }
                    PossibleStage::CheckerMoved(checker_moved) => {
                        PossibleStage::CheckerMoved(checker_moved)
                    }
                    PossibleStage::NoPossibleMoves(no_possible_moves) => {
                        PossibleStage::NoPossibleMoves(no_possible_moves)
                    }
                    PossibleStage::OutOfMoves(out_of_moves) => {
                        PossibleStage::OutOfMoves(out_of_moves)
                    }
                    PossibleStage::MovesCommited(moves_commited) => {
                        PossibleStage::MovesCommited(moves_commited)
                    }
                    PossibleStage::SideSwitched(side_switched) => {
                        PossibleStage::SideSwitched(side_switched)
                    }
                    PossibleStage::Win(win) => {
                        PossibleStage::Win(win)
                    }
                }
            }
            Event::Key(Key::Left) => {
                current_stage = match current_stage {
                    PossibleStage::DicesThrown(mut dices_thrown_stage) => {
                        if let Some(focused_pip) = dices_thrown_stage.focused_pip() {
                            dices_thrown_stage.focus_pip(Pip::new(*focused_pip + 1));

                            let dices_thrown_stage_view: StageView = StageView::from(&dices_thrown_stage, stage_theme, Side::White);
                            view_sender.clone().send(dices_thrown_stage_view).unwrap_or(());
                        }

                        PossibleStage::DicesThrown(dices_thrown_stage)
                    }
                    _ => { current_stage }
                }
            }
            Event::Key(Key::Right) => {
                current_stage = match current_stage {
                    PossibleStage::DicesThrown(mut dices_thrown_stage) => {
                        if let Some(focused_pip) = dices_thrown_stage.focused_pip() {
                            dices_thrown_stage.focus_pip(Pip::new(*focused_pip - 1));

                            let dices_thrown_stage_view: StageView = StageView::from(&dices_thrown_stage, stage_theme, Side::White);
                            view_sender.clone().send(dices_thrown_stage_view).unwrap_or(());
                        }

                        PossibleStage::DicesThrown(dices_thrown_stage)
                    }
                    _ => { current_stage }
                }
            }
            _ => {}
        }
    }
}
