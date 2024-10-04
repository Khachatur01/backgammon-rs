use std::thread;
use crate::backgammon::Backgammon;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{DummyView, LinearLayout, OnEventView, TextView};
use cursive::{event, Cursive};

pub fn open_local_game_page(cursive: &mut Cursive) {
    cursive.pop_layer();

    let (backgammon, event_sender, view_receiver) = Backgammon::new();
    backgammon.start();

    let board_layout =
        OnEventView::new(
            LinearLayout::vertical()
                .with_name("board")
        ).on_event(event::Key::Enter, move |cursive| {
            event_sender.send(event::Key::Enter).unwrap()
        });

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

    thread::spawn(move || {
        while let Ok(stage_view) = view_receiver.recv() {
            // &cursive.call_on_name("board", |linear_layout: &mut LinearLayout| {
                // linear_layout.remove_child(0);
                // linear_layout.add_child(stage_view);
            // });
        }
    });
}
