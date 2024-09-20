mod theme_builder;
mod page;
mod custom_view;
mod stage_theme;

use crate::page::main_menu::open_main_menu_page;
use crate::theme_builder::default;
use cursive::traits::Resizable;
use cursive::{CursiveRunnable, With};

fn main() {
    let mut cursive: CursiveRunnable = cursive::default();

    cursive.set_theme(default());

    open_main_menu_page(&mut cursive);

    cursive.run();
}
