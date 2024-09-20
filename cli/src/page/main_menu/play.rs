mod local_game;
mod network_game;
mod play_with_computer_page;

use crate::page::main_menu::open_main_menu_page;
use crate::page::main_menu::play::local_game::open_local_game_page;
use crate::page::main_menu::play::network_game::open_network_game_page;
use crate::page::main_menu::play::play_with_computer_page::open_play_with_computer_page;
use cursive::traits::Resizable;
use cursive::views::{Button, LinearLayout};
use cursive::Cursive;

pub fn open_play_page(cursive: &mut Cursive) {
    cursive.pop_layer();

    let horizontal_layout =
        LinearLayout::vertical()
            .child(Button::new("Local game", open_local_game_page))
            .child(Button::new("Network game", open_network_game_page))
            .child(Button::new("Play with computer", open_play_with_computer_page))
            .child(Button::new("Back", open_main_menu_page))
            .full_screen();

    cursive.add_layer(horizontal_layout);
}
