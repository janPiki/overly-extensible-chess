use crate::board::Board;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl Piece {
    pub fn generate_legal_moves(&self, from: (u32, u32), board: &Board) -> Vec<(u32, u32)> {
        match self.piece_type {
            PieceType::Pawn => generate_pawn_moves(self.color, from, board),
            PieceType::Knight => generate_knight_moves(self.color, from, board),
            PieceType::Bishop => generate_sliding_moves(from, board, BISHOP_DIRS, self.color),
            PieceType::Rook => generate_sliding_moves(from, board, ROOK_DIRS, self.color),
            PieceType::Queen => generate_sliding_moves(from, board, QUEEN_DIRS, self.color),
            PieceType::King => generate_step_moves(from, board, KING_DIRS, self.color),
        }
    }
}

const KING_DIRS: &[(i32, i32)] = &[
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn generate_step_moves(
    from: (u32, u32),
    board: &Board,
    directions: &[(i32, i32)],
    color: PieceColor,
) -> Vec<(u32, u32)> {
    let mut moves = Vec::new();

    for (dx, dy) in directions {
        let x = from.0 as i32 + dx;
        let y = from.1 as i32 + dy;

        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            let dest = (x as u32, y as u32);
            if let Some(node) = board.get_node(dest) {
                if let Some(piece) = node.piece {
                    if piece.color != color {
                        moves.push(dest);
                    }
                } else {
                    // No piece blocking
                    moves.push(dest);
                }
            }
        }
    }

    moves
}

const KNIGHT_DIRS: &[(i32, i32)] = &[
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
    (2, -1),
];

fn generate_knight_moves(color: PieceColor, from: (u32, u32), board: &Board) -> Vec<(u32, u32)> {
    generate_step_moves(from, board, KNIGHT_DIRS, color)
}

const ROOK_DIRS: &[(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: &[(i32, i32)] = &[(1, 1), (1, -1), (-1, 1), (-1, -1)];
const QUEEN_DIRS: &[(i32, i32)] = &[
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn generate_sliding_moves(
    from: (u32, u32),
    board: &Board,
    directions: &[(i32, i32)],
    color: PieceColor,
) -> Vec<(u32, u32)> {
    let mut moves = Vec::new();

    for (dx, dy) in directions {
        let mut x = from.0 as i32 + dx;
        let mut y = from.1 as i32 + dy;

        while x >= 0 && x < 8 && y >= 0 && y < 8 {
            let pos = (x as u32, y as u32);

            if let Some(node) = board.get_node(pos) {
                if let Some(piece) = node.piece {
                    if piece.color != color {
                        moves.push(pos);
                        break;
                    } else {
                        // Friendly blocks
                        break;
                    }
                } else {
                    // No piece
                    moves.push(pos);
                }
            }

            x += dx;
            y += dy;
        }
    }

    moves
}

fn generate_pawn_moves(color: PieceColor, from: (u32, u32), board: &Board) -> Vec<(u32, u32)> {
    let mut moves = Vec::new();
    let (x, y) = from;
    let dir: i32 = if color == PieceColor::White { -1 } else { 1 };
    let start_row = if color == PieceColor::White { 6 } else { 1 };

    let new_y_i32 = y as i32 + dir;
    if new_y_i32 >= 0 && new_y_i32 < 8 {
        let new_y = new_y_i32 as u32;

        if let Some(node) = board.get_node((x, new_y)) {
            if node.piece.is_none() {
                moves.push((x, new_y));

                // Double move at start
                if y == start_row {
                    let double_y = (y as i32 + 2 * dir) as u32;
                    if let Some(node) = board.get_node((x, double_y)) {
                        if node.piece.is_none() {
                            moves.push((x, double_y));
                        }
                    }
                }
            }
        }

        // Captures
        for dx in [-1, 1] {
            let nx = x as i32 + dx;
            if nx >= 8 || nx < 0 || new_y_i32 >= 8 || new_y_i32 < 0 {
                continue;
            }
            let capture_pos = (nx as u32, new_y);
            if let Some(node) = board.get_node(capture_pos) {
                if let Some(enemy) = node.piece {
                    if enemy.color != color {
                        moves.push(capture_pos);
                    }
                }
            }
        }
    }

    moves
}
