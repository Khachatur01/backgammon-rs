use cursive::align::Align;
use cursive::Cursive;
use cursive::traits::Resizable;
use cursive::views::{Button, DummyView, LinearLayout, TextView};
use play::open_play_page;
use rules::open_rules_page;
use settings::open_settings_page;

pub mod play;
pub mod rules;
pub mod settings;

pub fn open_main_menu_page(s: &mut Cursive) {
    s.pop_layer();

    let horizontal_layout =
        LinearLayout::vertical()
            .child(TextView::new("Welcome to backgammon game").align(Align::top_center()))
            .child(DummyView)
            .child(Button::new("Play", open_play_page))
            .child(Button::new("Rules", open_rules_page))
            .child(Button::new("Settings", open_settings_page))
            .child(Button::new("Exit", |s| { s.quit() }))
            .full_screen();

    s.add_layer(horizontal_layout);
}
