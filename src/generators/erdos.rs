extern crate rand;

use Generator;
use Edge;

use self::rand::Rng;

///
/// Constructs an [Erdős–Rényi random
/// graph](https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model)
/// with **n** nodes and an edge probability
/// of **p**.
///
pub struct Erdos {
    n: usize,
    p: f64,
}

impl Erdos {
    pub fn new(n: usize, p: f64) -> Erdos {
        Erdos { n, p }
    }
}

impl Generator for Erdos {
    fn edges<'a>(&'a self) -> Box<Iterator<Item=Edge> + 'a> {
        let mut rng = rand::thread_rng();

        Box::new((0..self.n).flat_map(move |u| (0..self.n).map(move |v| (u, v))).filter(move |_| rng.gen::<f64>() < self.p).map(|(u, v)| Edge::new(u,v)))
    }
}

#[cfg(test)]
mod tests {
    use StaticGraph;

    use representations::EdgeList;
    use representations::AdjacencyList;
    use generators::Erdos;
    use algorithms::breadth_first_search;

    #[test]
    fn creation_edgelist() {
        let erdos = Erdos::new(1000, 0.5);
        let graph = EdgeList::from_generator(&erdos);

        // Very unlikely to be wrong without implementation bug
        assert!(graph.num_edges() > 450_000);
        assert!(graph.num_edges() < 550_000);
    }

    #[test]
    fn creation_adjlist() {
        let erdos = Erdos::new(1000, 0.5);
        let graph = AdjacencyList::from_generator(&erdos);

        // Very unlikely to be wrong without implementation bug
        assert!(graph.num_edges() > 450_000);
        assert!(graph.num_edges() < 550_000);
    }

    #[test]
    fn bfs_edgelist() {
        let erdos = Erdos::new(1000, 0.01);
        let graph = EdgeList::from_generator(&erdos);

        let pred = breadth_first_search(&graph, 0);

        assert!(graph.num_edges() > 9_000);
        assert!(graph.num_edges() < 11_000);
        assert!(pred[0].is_some());
        assert_eq!(pred[0].unwrap(), 0);
    }

    #[test]
    fn bfs_adjacencylist() {
        let erdos = Erdos::new(1000, 0.01);
        let graph = AdjacencyList::from_generator(&erdos);

        let pred = breadth_first_search(&graph, 0);

        assert!(graph.num_edges() > 9_000);
        assert!(graph.num_edges() < 11_000);
        assert!(pred[0].is_some());
        assert_eq!(pred[0].unwrap(), 0);
    }
}
