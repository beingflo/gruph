use AccessGraph;
use Graph;
use Node;
use Edge;

use std::cmp;

pub struct AdjacencyList {
    adj: Vec<Vec<Node>>,
    num_nodes: usize,
}

impl<'a> AccessGraph<'a> for AdjacencyList {
    type NeighborIterator = Box<Iterator<Item=Node> + 'a>;
    type EdgeIterator = Box<Iterator<Item=Edge> + 'a>;

    fn has_edge(&self, from: Node, to: Node) -> bool {
        if self.adj.len() <= from {
            return false;
        }

        for u in &self.adj[from] {
            if *u == to {
                return true;
            }
        }

        false
    }

    fn neighbors(&'a self, vertex: Node) -> Self::NeighborIterator {
        Box::new(self.adj[vertex].iter().map(|&v| v))
    }

    fn edges(&'a self) -> Self::EdgeIterator {
        Box::new(self.adj.iter().enumerate().flat_map(|(u, vec)| vec.iter().map(move |v| Edge::new(u, *v))))
    }

    fn num_nodes(&self) -> usize {
        if self.adj.len() == 0 {
            0
        } else {
            self.num_nodes + 1
        }
    }

    fn num_edges(&self) -> usize {
        let mut num_edges = 0;
        for v in &self.adj {
            num_edges += v.len();
        }

        num_edges
    }

}

impl<'a> Graph<'a> for AdjacencyList {
    fn new() -> Self {
        AdjacencyList { adj: vec![], num_nodes: 0 }
    }

    fn add_edge(&mut self, from: Node, to: Node) {
        if self.adj.len() <= from {
            while self.adj.len() <= from {
                self.adj.push(vec![]);
            }
        }

        self.adj[from].push(to);

        self.num_nodes = cmp::max(self.num_nodes, from);
        self.num_nodes = cmp::max(self.num_nodes, to);
    }

    fn clear(&mut self) {
        self.adj.clear();
        self.num_nodes = 0;
    }
}

#[cfg(test)]
mod tests {
    use AccessGraph;
    use Graph;
    use Node;
    use Edge;

    use representations::AdjacencyList;

    #[test]
    fn creation() {
        let mut graph = AdjacencyList::new();
        graph.add_edge(0,1);
        assert!(graph.has_edge(0, 1));
    }

    #[test]
    fn duplicate_edge() {
        let mut graph = AdjacencyList::new();
        graph.add_edge(0,1);
        graph.add_edge(0,1);
        graph.add_edge(0,1);
        graph.add_edge(0,1);
        assert!(graph.has_edge(0, 1));
    }

    #[test]
    fn reverse_edge() {
        let mut graph = AdjacencyList::new();
        graph.add_edge(0,1);
        graph.add_edge(1,2);
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 2));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(2, 1));
    }

    #[test]
    fn neighbors() {
        let mut graph = AdjacencyList::new();
        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(0,3);
        graph.add_edge(1,2);

        assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,3]);
    }

    #[test]
    fn multi_neighbors() {
        let mut graph = AdjacencyList::new();
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
        let mut graph = AdjacencyList::new();
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
        let mut graph = AdjacencyList::new();
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
        let graph = AdjacencyList::new();
        assert_eq!(graph.num_nodes(), 0);
    }

    #[test]
    fn num_nodes_large() {
        let mut graph = AdjacencyList::new();

        for u in 0..100 {
            for v in 0..100 {
                graph.add_edge(u,v);
            }
        }

        assert_eq!(graph.num_nodes(), 100);
    }

    #[test]
    fn clear() {
        let mut graph = AdjacencyList::new();

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
