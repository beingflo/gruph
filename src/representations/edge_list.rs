use Generator;
use StaticGraph;
use Graph;
use Node;
use Edge;

use std::cmp;

#[derive(Clone, Debug)]
pub struct EdgeList {
    edges: Vec<Edge>,
}

impl Generator for EdgeList {
    fn edges<'a>(&'a self) -> Box<Iterator<Item=Edge> + 'a> {
        Box::new(self.edges.iter().map(|&e| e))
    }
}

impl StaticGraph for EdgeList {
    fn from_generator<T: Generator>(gen: &T) -> Self {
        let mut el = EdgeList::new();

        for e in gen.edges() {
            el.add_edge(e.u(), e.v());
        }

        el
    }

    fn num_nodes(&self) -> usize {
        let mut max_node = 0;
        for e in &self.edges {
            max_node = cmp::max(max_node, e.u());
            max_node = cmp::max(max_node, e.v());
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

    fn has_edge(&self, u: Node, v: Node) -> bool {
        for edge in &self.edges {
            if edge.u() == u && edge.v() == v {
                return true;
            }
        }

        false
    }

    fn neighbors<'a>(&'a self, vertex: Node) -> Box<Iterator<Item=Node> + 'a> {
        Box::new(self.edges.iter().filter(move |e| e.u() == vertex).map(|e| e.v()))
    }

    fn clear(&mut self) {
        self.edges.clear();
    }
}

impl Graph for EdgeList {
    fn new() -> Self {
        EdgeList { edges: vec![] }
    }

    fn add_edge(&mut self, u: Node, v: Node) {
        self.edges.push(Edge::new(u, v));
    }
}
