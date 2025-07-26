mod board;
mod game;
mod pieces;
mod render;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(720, 720)
        .title("Overly Extensible Chess")
        .build();

    let sprites_list = render::load_sprites(&mut rl, &thread);

    let tile_size = 90;
    let game = game::Game::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        render::draw_grid(tile_size, &mut d);
        render::draw_pieces(&mut d, &game.board, &sprites_list, tile_size, 720);
    }
}
