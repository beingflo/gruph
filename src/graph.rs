pub trait Graph<'a> {
    type Node;
    type NeighborIterator: Iterator<Item=Self::Node>;

    fn add_edge(&mut self, from: Self::Node, to: Self::Node);
    fn has_edge(&self, from: Self::Node, to: Self::Node) -> bool;
    fn neighbors(&'a self, from: Self::Node) -> Self::NeighborIterator;
}
