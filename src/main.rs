mod board;
mod game;
mod pieces;
mod render;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 640)
        .title("Singularity Chess")
        .build();

    let tile_size = 80;
    let game = game::Game::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        render::draw_grid(tile_size, &mut d);
    }
}
