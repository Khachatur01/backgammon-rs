use cursive::traits::Resizable;
use cursive::views::{DummyView, LinearLayout, TextView};
use cursive::Cursive;
use engine::stage::start::Start;
use engine::start_game;
use crate::custom_view::stage_view::StageView;
use crate::stage_theme::half_width::HalfWidth;
use crate::stage_theme::height::Height;
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
        height: Height::new(30),
    };

    let start_stage: Start = start_game();
    let start_stage_view: StageView = StageView::from(start_stage, stage_theme);

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

    cursive.add_layer(vertical_layout);
}
