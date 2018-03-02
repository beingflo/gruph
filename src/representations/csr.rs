use Generator;
use StaticGraph;
use Node;
use Edge;

#[derive(Clone)]
pub struct Csr {
    col: Vec<Node>,
    row: Vec<usize>,
}

impl Generator for Csr {
    fn edges<'a>(&'a self) -> Box<Iterator<Item=Edge> + 'a> {
        unimplemented!();
    }
}

impl StaticGraph for Csr {
    fn from_generator<T: Generator>(gen: &T) -> Self {
        unimplemented!();
    }

    fn num_nodes(&self) -> usize {
        unimplemented!();
    }

    fn num_edges(&self) -> usize {
        unimplemented!();
    }

    fn has_edge(&self, from: Node, to: Node) -> bool {
        unimplemented!();
    }

    fn neighbors<'a>(&'a self, vertex: Node) -> Box<Iterator<Item=Node> + 'a> {
        unimplemented!();
    }

    fn clear(&mut self) {
        unimplemented!();
    }
}
