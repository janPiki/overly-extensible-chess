use crate::board::Board;

pub struct Game {
    board: Board,
    turn: bool, // True for white, false for black
    move_count: u32,
    selected: Option<(u32, u32)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new_empty(),
            turn: true,
            move_count: 0,
            selected: None,
        }
    }
}
