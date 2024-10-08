use crate::backgammon;
use crate::custom_view::event_handler_view::EventHandlerView;
use crate::custom_view::stage_view::render_for::RenderFor;
use crate::custom_view::stage_view::StageView;
use crate::stage_theme::half_width::HalfWidth;
use crate::stage_theme::height::Height;
use crate::stage_theme::percent::Percent;
use crate::stage_theme::StageTheme;
use cursive::event::Event;
use cursive::traits::Resizable;
use cursive::views::{DummyView, LinearLayout, TextView};
use cursive::Cursive;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn open_local_game_page(cursive: &mut Cursive) {
    cursive.pop_layer();
    cursive.set_autorefresh(true);

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
        pip_size: 3,
        bore_off_column_width: 3,
        height: Height::new(23),
        peaces_cut_off_height_percent: Percent::new(35),
    };

    let (event_sender, view_receiver) = backgammon::new(stage_theme, RenderFor::ActiveSide);

    let mut current_stage_view: Arc<Mutex<Option<StageView>>> = Arc::new(Mutex::new(None));

    let mut current_stage_view_clone = current_stage_view.clone();

    thread::spawn(move || {
        while let Ok(stage_view) = view_receiver.recv() {
            current_stage_view_clone.lock().unwrap().replace(stage_view);
        }
    });

    let board_layout = EventHandlerView::new(
        LinearLayout::vertical(),
        move |event: Event, linear_layout: &mut LinearLayout| {
            match event {
                Event::Refresh => {
                    if let Ok(mut current_stage_view) = current_stage_view.lock() {
                        if let Some(current_stage_view) = current_stage_view.take() {
                            linear_layout.clear();

                            linear_layout.add_child(current_stage_view);
                        }
                    }
                }
                _ => {}
            }

            event_sender.send(event).unwrap();
        }
    );

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
}
