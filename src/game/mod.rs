use crate::board::Board;

pub struct Game {
    pub board: Board,
    pub turn: bool, // True for white, false for black
    pub move_count: u32,
    pub selected: Option<(u32, u32)>,
    pub hovered: Option<(u32, u32)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new_standard(),
            turn: true,
            move_count: 0,
            selected: None,
            hovered: None,
        }
    }
}
