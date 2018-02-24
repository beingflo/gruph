extern crate rand;

use self::rand::Rng;

use Graph;

///
/// Constructs an [Erdős–Rényi random
/// graph](https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93R%C3%A9nyi_model)
/// with **n** nodes and an edge probability
/// of **p**.
///
pub fn erdos<'a, T: Graph<'a>>(graph: &'a mut T, n: usize, p: f64) -> usize {
    graph.clear();

    let mut rng = rand::thread_rng();

    let mut num_edges = 0;

    for u in 0..n {
        for v in 0..n {
            if rng.gen::<f64>() < p {
                graph.add_edge(u, v);
                num_edges += 1;
            }
        }
    }

    num_edges
}

#[cfg(test)]
mod tests {
    use representations::EdgeList;
    use representations::AdjacencyList;
    use generators::erdos;

    #[test]
    fn creation_edgelist() {
        let mut graph = EdgeList::new();
        let edges = erdos(&mut graph, 1000, 0.5);

        // Very unlikely to be wrong without implementation bug
        assert!(edges > 450_000);
        assert!(edges < 550_000);
    }

    #[test]
    fn creation_adjlist() {
        let mut graph = AdjacencyList::new();
        let edges = erdos(&mut graph, 1000, 0.5);

        // Very unlikely to be wrong without implementation bug
        assert!(edges > 450_000);
        assert!(edges < 550_000);
    }
}
