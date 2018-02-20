type Node<T> = T;

struct Edge<T> {
    from:   Node<T>,
    to:     Node<T>,
}

impl<T> Edge<T> {
    fn new(from: Node<T>, to: Node<T>) -> Self {
        Edge { from, to }
    }
}

pub struct EdgeList<T> {
    edges: Vec<Edge<T>>,
}

impl<T: PartialEq + Ord + Copy + Sized> EdgeList<T> {
    pub fn new() -> Self {
        EdgeList::<T> { edges: vec![] }
    }

    pub fn with_capacity(edges: usize) -> Self {
        EdgeList::<T> { edges: Vec::with_capacity(edges) }
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn add_edge(&mut self, from: Node<T>, to: Node<T>) {
        self.edges.push(Edge::new(from, to));
    }

    pub fn has_edge(&self, from: Node<T>, to: Node<T>) -> bool {
        for edge in &self.edges {
            if edge.from == from && edge.to == to {
                return true;
            }
        }

        false
    }

    pub fn neighbors(&self, vertex: Node<T>) -> Vec<Node<T>> {
        let mut neighbors: Vec<Node<T>> = self.edges.iter().filter(|e| e.from == vertex).map(|e| e.to).collect();
        neighbors.sort();
        neighbors.dedup();

        neighbors
    }
}


#[cfg(test)]
mod tests {
    use EdgeList;

    #[test]
    fn creation() {
        let mut graph = EdgeList::new();
        graph.add_edge(0,1);
        assert!(graph.has_edge(0, 1));
    }

    #[test]
    fn duplicate_edge() {
        let mut graph = EdgeList::new();
        graph.add_edge(0,1);
        graph.add_edge(0,1);
        graph.add_edge(0,1);
        graph.add_edge(0,1);
        assert!(graph.has_edge(0, 1));
    }

    #[test]
    fn reverse_edge() {
        let mut graph = EdgeList::new();
        graph.add_edge(0,1);
        graph.add_edge(1,2);
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(2, 1));
    }

    #[test]
    fn neighbors() {
        let mut graph = EdgeList::new();
        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(0,3);
        graph.add_edge(1,2);

        assert_eq!(graph.neighbors(0), vec![1,2,3]);
    }

    #[test]
    fn dedup_neighbors() {
        let mut graph = EdgeList::new();
        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(0,2);
        graph.add_edge(0,3);
        graph.add_edge(0,3);
        graph.add_edge(0,3);
        graph.add_edge(1,2);

        assert_eq!(graph.neighbors(0), vec![1,2,3]);
    }
}
