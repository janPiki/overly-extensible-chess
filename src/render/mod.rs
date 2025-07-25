use crate::board::Board;
use crate::pieces::{PieceColor, PieceType};
use raylib::prelude::*;
use std::collections::HashMap;

struct SpriteList {
    sprites: HashMap<(PieceColor, PieceType), Texture2D>,
}

pub fn load_sprites() {
    // TODO:
}

pub fn draw_grid(tile_size: i32, d: &mut RaylibDrawHandle) {
    for rank in 0..8 {
        for file in 0..8 {
            let tile_color = if (rank % 2 == 0 && file % 2 == 0) || (rank % 2 != 0 && file % 2 != 0)
            {
                Color::LIGHTBLUE
            } else {
                Color::RAYWHITE
            };
            d.draw_rectangle(
                file * tile_size,
                rank * tile_size,
                tile_size,
                tile_size,
                tile_color,
            );
        }
    }
}

pub fn draw_board(board: &Board) {
    // TODO:
}
