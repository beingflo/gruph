use AccessGraph;
use Graph;
use Node;
use Edge;

use std::cmp;

pub struct EdgeList {
    edges: Vec<Edge>,
}

impl<'a> AccessGraph<'a> for EdgeList {
    type NeighborIterator = Box<Iterator<Item=Node> + 'a>;
    type EdgeIterator = Box<Iterator<Item=Edge> + 'a>;

    fn has_edge(&self, from: Node, to: Node) -> bool {
        for edge in &self.edges {
            if edge.source() == from && edge.target() == to {
                return true;
            }
        }

        false
    }

    fn neighbors(&'a self, vertex: Node) -> Self::NeighborIterator {
        Box::new(self.edges.iter().filter(move |e| e.source() == vertex).map(|e| e.target()))
    }

    fn edges(&'a self) -> Self::EdgeIterator {
        Box::new(self.edges.iter().map(|&e| e))
    }

    fn num_nodes(&self) -> usize {
        let mut max_node = 0;
        for e in &self.edges {
            max_node = cmp::max(max_node, e.source());
            max_node = cmp::max(max_node, e.target());
        }

        if self.edges.len() > 0 {
            max_node + 1
        } else {
            0
        }
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }
}

impl<'a> Graph<'a> for EdgeList {
    fn new() -> Self {
        EdgeList { edges: vec![] }
    }

    fn add_edge(&mut self, from: Node, to: Node) {
        self.edges.push(Edge::new(from, to));
    }

    fn clear(&mut self) {
        self.edges.clear();
    }
}

#[cfg(test)]
mod tests {
    use AccessGraph;
    use Graph;
    use Node;
    use Edge;

    use representations::EdgeList;

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

        assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,3]);
    }

    #[test]
    fn multi_neighbors() {
        let mut graph = EdgeList::new();
        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(0,2);
        graph.add_edge(0,3);
        graph.add_edge(0,3);
        graph.add_edge(0,3);
        graph.add_edge(1,2);

        assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,2,3,3,3]);
    }

    #[test]
    fn edges() {
        let mut graph = EdgeList::new();
        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(0,2);
        graph.add_edge(0,3);
        graph.add_edge(0,3);
        graph.add_edge(0,3);
        graph.add_edge(1,2);

        assert_eq!(graph.edges().collect::<Vec<Edge>>()[6], Edge::new(1,2));
        assert_eq!(graph.edges().collect::<Vec<Edge>>().len(), 7);
    }

    #[test]
    fn num_nodes() {
        let mut graph = EdgeList::new();
        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(0,2);
        graph.add_edge(0,3);
        graph.add_edge(0,3);
        graph.add_edge(0,3);
        graph.add_edge(1,2);

        assert_eq!(graph.num_nodes(), 4);
    }

    #[test]
    fn num_nodes_empty() {
        let graph = EdgeList::new();
        assert_eq!(graph.num_nodes(), 0);
    }

    #[test]
    fn num_nodes_large() {
        let mut graph = EdgeList::new();

        for u in 0..100 {
            for v in 0..100 {
                graph.add_edge(u,v);
            }
        }

        assert_eq!(graph.num_nodes(), 100);
    }

    #[test]
    fn clear() {
        let mut graph = EdgeList::new();

        for u in 0..100 {
            for v in 0..100 {
                graph.add_edge(u,v);
            }
        }

        assert_eq!(graph.num_nodes(), 100);
        graph.clear();
        assert_eq!(graph.num_nodes(), 0);
    }
}
