use algorithms::*;

pub type Node = usize;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Edge(Node, Node);

impl Edge {
    pub fn new(from: Node, to: Node) -> Self {
        Edge(from, to)
    }

    pub fn source(&self) -> Node {
        self.0
    }

    pub fn target(&self) -> Node {
        self.1
    }
}

pub trait Generator {
    fn edges<'a>(&'a self) -> Box<Iterator<Item=Edge> + 'a>;
}

pub trait StaticGraph: Generator {
    fn from_generator<T: Generator>(gen: &T) -> Self;

    fn num_nodes(&self) -> usize;
    fn num_edges(&self) -> usize;

    fn has_edge(&self, from: Node, to: Node) -> bool;

    fn neighbors<'a>(&'a self, from: Node) -> Box<Iterator<Item=Node> + 'a>;

    fn breadth_first_search(&self, start: Node) -> Vec<Option<Node>> where Self: Sized {
        breadth_first_search(self, start)
    }

    fn clear(&mut self);
}

pub trait Graph : StaticGraph {
    fn new() -> Self;
    fn add_edge(&mut self, from: Node, to: Node);
}
