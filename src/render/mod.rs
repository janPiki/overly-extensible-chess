use std::collections::HashMap;

use crate::board::Board;
use crate::pieces::{PieceColor, PieceType};
use raylib::prelude::*;

pub type SpriteMap = HashMap<(PieceColor, PieceType), Texture2D>;

pub fn load_sprites(rl: &mut RaylibHandle, thread: &RaylibThread) -> SpriteMap {
    use PieceColor::*;
    use PieceType::*;

    let colors = [White, Black];
    let types = [Pawn, Knight, Bishop, Rook, Queen, King];

    let mut sprites = HashMap::new();

    for &color in &colors {
        for &piece_type in &types {
            let file_name = format!(
                "assets/{}-{}.png",
                match color {
                    White => "white",
                    Black => "black",
                },
                match piece_type {
                    Pawn => "pawn",
                    Knight => "knight",
                    Bishop => "bishop",
                    Rook => "rook",
                    Queen => "queen",
                    King => "king",
                }
            );

            let texture = rl.load_texture(thread, &file_name).unwrap();
            sprites.insert((color, piece_type), texture);
        }
    }

    sprites
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

pub fn draw_pieces(
    d: &mut RaylibDrawHandle,
    board: &Board,
    sprites: &SpriteMap,
    tile_size: i32,
    screen_size: i32,
) {
    for node in &board.nodes {
        if let Some(piece) = node.piece {
            let (x, y) = node.vector;
            let x_pos = x as i32 * tile_size;
            let y_pos = y as i32 * tile_size;
            if let Some(texture) = sprites.get(&(piece.color, piece.piece_type)) {
                d.draw_texture(
                    texture,
                    x_pos,
                    (screen_size - y_pos) - tile_size,
                    Color::WHITE,
                );
            }
        }
    }
}
