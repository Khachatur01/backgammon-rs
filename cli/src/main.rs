mod theme_builder;

use crate::theme_builder::default;
use cursive::align::Align;
use cursive::traits::Resizable;
use cursive::views::{Button, DummyView, LinearLayout, TextView};
use cursive::{CursiveRunnable, With};

fn main() {
    let mut cursive: CursiveRunnable = cursive::default();

    cursive.set_theme(default());

    let horizontal_layout =
        LinearLayout::vertical()
            .child(TextView::new("Welcome to backgammon game").align(Align::top_center()))
            .child(DummyView)
            .child(Button::new("Play", |s| { s.pop_layer(); }))
            .child(Button::new("Rules", |s| { s.pop_layer(); }))
            .child(Button::new("Settings", |s| { s.pop_layer(); }))
            .child(Button::new("Exit", |s| { s.quit() }))
            .full_screen();

    cursive.add_layer(horizontal_layout);

    cursive.run();
}
