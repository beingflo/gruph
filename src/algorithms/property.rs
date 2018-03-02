use StaticGraph;

pub fn is_bipartite<T: StaticGraph>(graph: &T) -> bool {
    unimplemented!()
}

pub fn has_cycle<T: StaticGraph>(graph: &T) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use StaticGraph;
    use Graph;

    use representations::EdgeList;
    use representations::AdjacencyList;

    #[test]
    fn simple() {
        let mut graph = EdgeList::new();

        assert_eq!(graph.is_bipartite(), true);
        assert_eq!(graph.has_cycle(), false);

        graph.add_edge(0,1);

        assert_eq!(graph.is_bipartite(), true);
        assert_eq!(graph.has_cycle(), false);

        graph.add_edge(1,2);

        assert_eq!(graph.is_bipartite(), true);
        assert_eq!(graph.has_cycle(), false);

        graph.add_edge(2,0);

        assert_eq!(graph.is_bipartite(), false);
        assert_eq!(graph.has_cycle(), true);
    }

    #[test]
    fn even_cycle() {
        let mut graph = EdgeList::new();

        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(1,3);
        graph.add_edge(2,3);

        assert_eq!(graph.is_bipartite(), true);
        assert_eq!(graph.has_cycle(), true);
    }

    #[test]
    fn large_bipartite() {
        let mut graph = AdjacencyList::new();

        for u in 0..50 {
            for v in 50..100 {
                graph.add_edge(u, v);
            }
        }

        assert_eq!(graph.is_bipartite(), true);
        assert_eq!(graph.has_cycle(), true);
    }


    #[test]
    fn large_notbipartite() {
        let mut graph = AdjacencyList::new();

        for u in 0..50 {
            for v in 50..100 {
                graph.add_edge(u, v);
            }
        }

        graph.add_edge(12, 42);

        assert_eq!(graph.is_bipartite(), true);
        assert_eq!(graph.has_cycle(), true);
    }
}
