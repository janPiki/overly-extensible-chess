use std::collections::HashMap;

use crate::board::Board;
use crate::game::Game;
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
    tile_size: i32,
    sprites: &SpriteMap,
    game: &Game,
) {
    for node in &board.nodes {
        if let Some(piece) = node.piece {
            if game
                .dragging
                .map_or(true, |dragged_pos| dragged_pos != node.vector)
            {
                let (x, y) = node.vector;
                let x_pos = x as i32 * tile_size;
                let y_pos = y as i32 * tile_size;
                if let Some(texture) = sprites.get(&(piece.color, piece.piece_type)) {
                    d.draw_texture(texture, x_pos, y_pos, Color::WHITE);
                }
            }
        }

        // Draw the piece being dragged at mouse cursor
        if let Some(piece) = game.dragged_piece {
            if let Some(texture) = sprites.get(&(piece.color, piece.piece_type)) {
                let mouse_pos = d.get_mouse_position();
                // Offset position to make it centered
                let x = mouse_pos.x - 45.0;
                let y = mouse_pos.y - 45.0;
                d.draw_texture(texture, x as i32, y as i32, Color::WHITE);
            }
        }
    }
}
