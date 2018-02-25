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

pub trait AccessGraph {
    fn num_nodes(&self) -> usize;
    fn num_edges(&self) -> usize;

    fn has_edge(&self, from: Node, to: Node) -> bool;

    fn neighbors<'a>(&'a self, from: Node) -> Box<Iterator<Item=Node> + 'a>;
    fn edges<'a>(&'a self) -> Box<Iterator<Item=Edge> + 'a>;
}

pub trait Graph : AccessGraph {
    fn new() -> Self;
    fn add_edge(&mut self, from: Node, to: Node);
    fn clear(&mut self);
}
