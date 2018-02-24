use std::cmp;

use Graph;
use Node;

struct Edge {
    from:   Node,
    to:     Node,
}

impl Edge {
    fn new(from: Node, to: Node) -> Self {
        Edge { from, to }
    }
}

pub struct EdgeList {
    edges: Vec<Edge>,
}

impl EdgeList {
    pub fn new() -> Self {
        EdgeList { edges: vec![] }
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }
}

impl<'a> Graph<'a> for EdgeList {
    type NeighborIterator = EdgeListNeighborIterator<'a>;

    fn add_edge(&mut self, from: Node, to: Node) {
        self.edges.push(Edge::new(from, to));
    }

    fn has_edge(&self, from: Node, to: Node) -> bool {
        for edge in &self.edges {
            if edge.from == from && edge.to == to {
                return true;
            }
        }

        false
    }

    fn neighbors(&'a self, vertex: Node) -> Self::NeighborIterator {
        EdgeListNeighborIterator::new(self, vertex)
    }

    fn num_nodes(&self) -> usize {
        let mut max_node = 0;
        for e in &self.edges {
            max_node = cmp::max(max_node, e.from);
            max_node = cmp::max(max_node, e.to);
        }

        if self.edges.len() > 0 {
            max_node + 1
        } else {
            0
        }
    }

    fn clear(&mut self) {
        self.edges.clear();
    }
}

pub struct EdgeListNeighborIterator<'a> {
    edges: &'a Vec<Edge>,
    node: Node,
    index: usize,
}

impl<'a> EdgeListNeighborIterator<'a> {
    fn new(edgelist: &'a EdgeList, node: Node) -> Self {
        EdgeListNeighborIterator { edges: edgelist.edges(), node, index: 0 }
    }
}

impl<'a> Iterator for EdgeListNeighborIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        for idx in self.index..self.edges.len() {
            if self.edges[idx].from == self.node {
                self.index = idx+1;
                return Some(&self.edges[idx].to)
            }
        }

        self.index = self.edges.len();
        None
    }
}

#[cfg(test)]
mod tests {
    use Graph;
    use Node;
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

        assert_eq!(graph.neighbors(0).collect::<Vec<&Node>>(), vec![&1,&2,&3]);
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

        assert_eq!(graph.neighbors(0).collect::<Vec<&Node>>(), vec![&1,&2,&2,&3,&3,&3]);
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
