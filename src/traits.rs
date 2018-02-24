pub type Node = usize;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

pub trait QueryGraph<'a> {
    type NeighborIterator: Iterator<Item=Node>;
    type EdgeIterator: Iterator<Item=Edge>;

    fn has_edge(&self, from: Node, to: Node) -> bool;
    fn neighbors(&'a self, from: Node) -> Self::NeighborIterator;

    fn num_nodes(&self) -> usize;
    fn num_edges(&self) -> usize;

    fn edges(&'a self) -> Self::EdgeIterator;
}

pub trait Graph<'a> : QueryGraph<'a> {
    fn add_edge(&mut self, from: Node, to: Node);
    fn clear(&mut self);
}
