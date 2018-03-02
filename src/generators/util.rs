use Generator;
use Edge;

pub struct CompleteGraph {
    n: usize,
}

impl CompleteGraph {
    pub fn new(n: usize) -> CompleteGraph {
        CompleteGraph { n }
    }
}

impl Generator for CompleteGraph {
    fn edges<'a>(&'a self) -> Box<Iterator<Item=Edge> + 'a> {
        Box::new((0..self.n).flat_map(move |u| (0..self.n).map(move |v| (u, v))).map(|(u, v)| Edge::new(u,v)))
    }
}

#[cfg(test)]
mod tests {
    use StaticGraph;

    use representations::EdgeList;
    use representations::AdjacencyList;
    use generators::CompleteGraph;
    use algorithms::breadth_first_search;

    #[test]
    fn creation_edgelist() {
        let gen = CompleteGraph::new(1000);
        let graph = EdgeList::from_generator(&gen);

        assert!(graph.num_edges() == 1_000_000);
    }

    #[test]
    fn creation_adjlist() {
        let gen = CompleteGraph::new(1000);
        let graph = AdjacencyList::from_generator(&gen);

        assert!(graph.num_edges() == 1_000_000);
    }

    #[test]
    fn bfs_edgelist() {
        let gen = CompleteGraph::new(100);
        let graph = EdgeList::from_generator(&gen);

        let pred = breadth_first_search(&graph, 0);

        assert!(graph.num_edges() == 10_000);
        assert!(pred[0].is_some());
        assert_eq!(pred[0].unwrap(), 0);
    }

    #[test]
    fn bfs_adjacencylist() {
        let gen = CompleteGraph::new(100);
        let graph = AdjacencyList::from_generator(&gen);

        let pred = breadth_first_search(&graph, 0);

        assert!(graph.num_edges() == 10_000);
        assert!(pred[0].is_some());
        assert_eq!(pred[0].unwrap(), 0);
    }
}
