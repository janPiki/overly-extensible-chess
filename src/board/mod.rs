use crate::pieces::Piece;

pub struct Node {
    edges: Vec<Edge>,
    vector: (u32, u32),
    piece: Option<Piece>,
}

pub struct Edge {
    node: (u32, u32),
    dis_vector: (u32, u32),
}

pub struct Board {
    nodes: Vec<Node>,
}

impl Board {
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

    fn get_node_mut(&mut self, vec: (u32, u32)) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.vector == vec)
    }

    fn get_node(&mut self, vec: (u32, u32)) -> Option<&Node> {
        self.nodes.iter().find(|n| n.vector == vec)
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
