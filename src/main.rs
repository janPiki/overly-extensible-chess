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
    let mut game = game::Game::new();

    while !rl.window_should_close() {
        game.update_game(&rl, tile_size);

        let mut d = rl.begin_drawing(&thread);
        render::draw_grid(tile_size as i32, &mut d);
        render::draw_pieces(&mut d, &game.board, tile_size as i32, &sprites_list, &game);
    }
}
