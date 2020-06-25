use std::fmt;

pub struct Edge {
    pub next: i32,
    pub to: i32,
}

pub struct Graph {
    pub edges: Vec<Edge>,
    pub head: Vec<i32>,
    pub edge_count: i32
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.head.len() {
            write!(f, "{} : ", i).unwrap();
            let mut e = self.head[i];
            while e != -1 {
                write!(f, "{} ", self.edges[e as usize].to).unwrap();
                e = self.edges[e as usize].next;
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

impl Graph {
    pub fn new(node_count: i32) -> Graph {
        Graph {
            edges: Vec::new(),
            head: vec![-1; node_count as usize],
            edge_count: 0
        }
    }

    pub fn add_edge(&mut self, from: i32, to: i32) {
        let edge = Edge {
            next: self.head[from as usize],
            to,
        };
        self.head[from as usize] = self.edge_count;
        self.edges.push(edge);
        self.edge_count += 1;
    }
}