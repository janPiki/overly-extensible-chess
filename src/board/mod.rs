use crate::pieces::Piece;

pub struct Node {
    pub edges: Vec<Edge>,
    pub vector: (u32, u32),
    pub piece: Option<Piece>,
}

pub struct Edge {
    pub node: (u32, u32),
    pub dis_vector: (u32, u32),
}

pub struct Board {
    pub nodes: Vec<Node>,
}

impl Board {
    pub fn new_empty() -> Self {
        let mut nodes = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                nodes.push(Node {
                    vector: (file, rank),
                    piece: None,
                    edges: Vec::new(),
                });
            }
        }

        let mut board = Board { nodes };

        for rank in 0..8 {
            for file in 0..8 {
                let vec = (file, rank);
                if let Some(node) = board.get_node_mut(vec) {
                    node.edges = Board::generate_adjacency_edges(vec);
                }
            }
        }
        board
    }

    pub fn new_standard() -> Self {
        let mut board = Board::new_empty();

        use crate::pieces::PieceColor::*;
        use crate::pieces::PieceType::*;

        let back_rank = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];

        // Setup white pieces
        for rank in 0..8 {
            board.set_piece(
                (rank, 7),
                Piece {
                    piece_type: back_rank[rank as usize],
                    color: White,
                },
            );
            board.set_piece(
                (rank, 6),
                Piece {
                    piece_type: Pawn,
                    color: White,
                },
            );
        }

        // Setup black pieces
        for rank in 0..8 {
            board.set_piece(
                (rank, 0),
                Piece {
                    piece_type: back_rank[rank as usize],
                    color: Black,
                },
            );
            board.set_piece(
                (rank, 1),
                Piece {
                    piece_type: Pawn,
                    color: Black,
                },
            );
        }

        board
    }

    pub fn get_node_mut(&mut self, pos: (u32, u32)) -> Option<&mut Node> {
        let (file, rank) = pos;
        if file < 8 && rank < 8 {
            let idx = (rank * 8 + file) as usize;
            self.nodes.get_mut(idx)
        } else {
            None
        }
    }

    pub fn get_node(&self, pos: (u32, u32)) -> Option<&Node> {
        let (file, rank) = pos;
        if file < 8 && rank < 8 {
            let idx = (rank * 8 + file) as usize;
            self.nodes.get(idx)
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, position: (u32, u32), piece: Piece) {
        if let Some(node) = self.get_node_mut(position) {
            node.piece = Some(piece);
        }
    }

    pub fn remove_piece(&mut self, target: (u32, u32)) {
        if let Some(node) = self.get_node_mut(target) {
            node.piece = None
        }
    }

    pub fn move_piece(&mut self, from: (u32, u32), to: (u32, u32)) -> bool {
        if let Some(node) = self.get_node_mut(from) {
            if let Some(piece) = node.piece {
                self.set_piece(to, piece);
                self.remove_piece(from);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn generate_adjacency_edges(pos: (u32, u32)) -> Vec<Edge> {
        let mut edges = Vec::new();
        let directions: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let (x, y) = (pos.0 as i8, pos.1 as i8);

        for (dx, dy) in directions.iter() {
            let nx = x + dx;
            let ny = y + dy;
            if ny >= 0 && ny < 8 && nx >= 0 && nx < 8 {
                edges.push(Edge {
                    node: (nx as u32, ny as u32),
                    dis_vector: (*dx as u32, *dy as u32),
                });
            }
        }

        edges
    }
}
