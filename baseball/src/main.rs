mod controller;
mod domain;
mod view;

use crate::controller::game_controller::GameController;
use crate::view::game_view::GameView;
use crate::view::input_view::ConsoleInputView;
use crate::view::output_view::ConsoleOutputView;
use std::io::{stdin, stdout, Write};
use std::ops::Not;

fn main() {
    let game_controller = GameController::new(GameView::new(
        Box::new(ConsoleInputView::new()),
        Box::new(ConsoleOutputView::new()),
    ));

    game_controller.game_init();
    game_controller.play();
}


fn read_line(msg: &str) -> String {
    print!("{}", msg);

    stdout().flush().unwrap();

    stdin().lines().next().unwrap().unwrap()
}
