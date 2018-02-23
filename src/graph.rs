pub type Node = usize;

pub trait Graph<'a> {
    type NeighborIterator: Iterator<Item=&'a Node>;

    fn add_edge(&mut self, from: Node, to: Node);
    fn has_edge(&self, from: Node, to: Node) -> bool;
    fn neighbors(&'a self, from: Node) -> Self::NeighborIterator;
    fn num_nodes(&self) -> usize;
}
