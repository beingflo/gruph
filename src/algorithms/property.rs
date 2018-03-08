use StaticGraph;

use std::collections::VecDeque;

pub fn is_bipartite<T: StaticGraph>(graph: &T) -> bool {
    if graph.num_nodes() <= 1 {
        return true;
    }

    let mut q = VecDeque::new();

    let mut color: Vec<Option<bool>> = vec![None; graph.num_nodes()];
    let mut visited = vec![false; graph.num_nodes()];

    visited[0] = true;
    color[0] = Some(true);

    q.push_front(0);

    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        for u in graph.neighbors(v) {
            if visited[u] == false {
                color[u] = Some(!color[v].unwrap());
                visited[u] = true;
                q.push_front(u);
            } else {
                if color[v] == color[u] {
                    return false;
                }
            }
        }
    }

    true
}

pub fn has_cycle<T: StaticGraph>(graph: &T) -> bool {
    if graph.num_nodes() <= 0 {
        return false;
    }

    let mut q = VecDeque::new();
    let mut visited = vec![false; graph.num_nodes()];

    visited[0] = true;

    q.push_front(0);

    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        for u in graph.neighbors(v) {
            if visited[u] == true {
                return true;
            }
            visited[u] = true;
            q.push_front(u);
        }
    }

    false
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
        assert_eq!(graph.has_cycle(), false);
    }


    #[test]
    fn large_notbipartite() {
        let mut graph = AdjacencyList::new();

        for u in 0..50 {
            for v in 50..100 {
                graph.add_edge(u, v);
            }
        }

        graph.add_edge(70, 89);
        graph.add_edge(70, 12);

        assert_eq!(graph.is_bipartite(), false);
        assert_eq!(graph.has_cycle(), true);
    }
}
