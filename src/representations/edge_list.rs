use AccessGraph;
use Graph;
use Node;
use Edge;

use std::cmp;

pub struct EdgeList {
    edges: Vec<Edge>,
}

impl AccessGraph for EdgeList {
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

    fn has_edge(&self, from: Node, to: Node) -> bool {
        for edge in &self.edges {
            if edge.source() == from && edge.target() == to {
                return true;
            }
        }

        false
    }

    fn neighbors<'a>(&'a self, vertex: Node) -> Box<Iterator<Item=Node> + 'a> {
        Box::new(self.edges.iter().filter(move |e| e.source() == vertex).map(|e| e.target()))
    }

    fn edges<'a>(&'a self) -> Box<Iterator<Item=Edge> + 'a> {
        Box::new(self.edges.iter().map(|&e| e))
    }

}

impl Graph for EdgeList {
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
