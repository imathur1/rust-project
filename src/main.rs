mod shared;
mod game;
mod snek;

use std::collections::HashMap;

use macroquad::prelude::*;

use game::Game;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snek".to_owned(),
        window_resizable: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new(
        WINDOW_WIDTH,  WINDOW_HEIGHT,
        20, 30, 20
    );
    game.spawn_snek(0, true);
    game.spawn_snek(1, false);
    game.spawn_snek(2, false);
    game.spawn_snek(3, false);
    loop {
        clear_background(BLACK);

        game.handle_events();
        game.update();

        next_frame().await
    }
}
