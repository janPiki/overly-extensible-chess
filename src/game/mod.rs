use raylib::prelude::*;

use crate::{
    board::Board,
    pieces::{Piece, PieceColor::*},
};

pub struct Game {
    pub board: Board,
    pub turn: bool, // True for white, false for black
    pub move_count: u32,
    pub dragging: Option<(u32, u32)>,
    pub dragged_piece: Option<Piece>,
    pub hovered: Option<(u32, u32)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new_standard(),
            turn: true,
            move_count: 0,
            dragging: None,
            dragged_piece: None,
            hovered: None,
        }
    }

    pub fn update_game(&mut self, rl: &RaylibHandle, tile_size: u32) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            let x = (mouse_pos.x / tile_size as f32).floor() as u32;
            let y = (mouse_pos.y / tile_size as f32).floor() as u32;
            let clicked = (x, y);

            if let Some(node) = self.board.get_node(clicked) {
                if let Some(piece) = node.piece {
                    if (self.turn && piece.color == White) || (!self.turn && piece.color == Black) {
                        self.dragging = Some(clicked);
                        self.dragged_piece = Some(piece);
                    }
                }
            }
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(from_pos) = self.dragging {
                let mouse_pos = rl.get_mouse_position();
                let x = (mouse_pos.x / tile_size as f32).floor() as u32;
                let y = (mouse_pos.y / tile_size as f32).floor() as u32;
                let to_pos = (x, y);

                if from_pos != to_pos {
                    if let Some(node) = self.board.get_node(from_pos) {
                        if let Some(piece) = node.piece {
                            let moves = piece.generate_legal_moves(from_pos, &self.board);
                            if moves.contains(&to_pos) {
                                if self.board.move_piece(from_pos, to_pos) {
                                    self.move_count += 1;
                                    self.turn = !self.turn;
                                }
                            }
                        }
                    }
                }

                self.dragging = None;
                self.dragged_piece = None;
            }
        }
    }
}
