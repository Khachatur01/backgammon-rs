use crate::custom_view::stage::StageView;
use cursive::traits::Resizable;
use cursive::views::{DummyView, LinearLayout, TextView};
use cursive::Cursive;
use engine::stage::start::Start;
use engine::start_game;

pub fn open_local_game_page(s: &mut Cursive) {
    s.pop_layer();

    let start_stage: Start = start_game();
    let start_stage_view: StageView = StageView::from(start_stage);

    let board_layout =
        LinearLayout::vertical()
            .child(start_stage_view);

    let information_layout =
        LinearLayout::vertical()
            .child(TextView::new("Move is for -> Black"));

    let vertical_layout =
        LinearLayout::horizontal()
            .child(board_layout)
            .child(DummyView)
            .child(information_layout)
            .full_screen();

    s.add_layer(vertical_layout);
}
