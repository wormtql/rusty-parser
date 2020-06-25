pub struct Edge<E> {
    pub next: i32,
    pub next_rev: i32,
    pub to: i32,
    pub from: i32,
    pub value: E
}


pub struct Graph<N, E> {
    pub head: Vec<i32>,
    pub edges: Vec<Edge<E>>,
    pub value: Vec<N>,

    pub parent: Vec<i32>
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Graph<N, E> {
        Graph {
            head: Vec::new(),
            edges: Vec::new(),
            value: Vec::new(),
            parent: Vec::new()
        }
    }

    pub fn add_node(&mut self, value: N) {
        self.head.push(-1);
        self.parent.push(-1);
        self.value.push(value);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, value: E) {
        let edge = Edge {
            next: self.head[from],
            next_rev: self.parent[to],
            to: to as i32,
            from: from as i32,
            value: value
        };

        self.edges.push(edge);
        self.head[from] = self.edges.len() as i32 - 1;
        self.parent[to] = self.edges.len() as i32 - 1;
    }
}