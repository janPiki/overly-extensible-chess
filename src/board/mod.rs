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
    // (0, 0) is top left, but rotated 90°
    // So x/files -> y/ranks
    // And y/ranks -> x/files
    pub fn new_empty() -> Self {
        let mut nodes = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                nodes.push(Node {
                    vector: (rank, file),
                    piece: None,
                    edges: Vec::new(),
                });
            }
        }

        let mut board = Board { nodes };

        for rank in 0..8 {
            for file in 0..8 {
                let vec = (rank, file);
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
                (rank, 0),
                Piece {
                    piece_type: back_rank[rank as usize],
                    color: White,
                },
            );
            board.set_piece(
                (rank, 1),
                Piece {
                    piece_type: Pawn,
                    color: White,
                },
            );
        }

        // Setup black pieces
        for rank in 0..8 {
            board.set_piece(
                (rank, 7),
                Piece {
                    piece_type: back_rank[rank as usize],
                    color: Black,
                },
            );
            board.set_piece(
                (rank, 6),
                Piece {
                    piece_type: Pawn,
                    color: Black,
                },
            );
        }

        board
    }

    fn get_node_mut(&mut self, vec: (u32, u32)) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.vector == vec)
    }

    fn get_node(&mut self, vec: (u32, u32)) -> Option<&Node> {
        self.nodes.iter().find(|n| n.vector == vec)
    }

    pub fn set_piece(&mut self, position: (u32, u32), piece: Piece) {
        if let Some(node) = self.get_node_mut(position) {
            node.piece = Some(piece);
        }
    }

    fn generate_adjacency_edges(vec: (u32, u32)) -> Vec<Edge> {
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

        for (dx, dy) in directions.iter() {
            let (ny, nx) = (vec.0 as i8 + dy, vec.1 as i8 + dx);
            if ny >= 0 && ny < 8 && nx >= 0 && nx < 8 {
                edges.push(Edge {
                    node: (ny as u32, nx as u32),
                    dis_vector: (*dy as u32, *dx as u32),
                });
            }
        }

        edges
    }
}
