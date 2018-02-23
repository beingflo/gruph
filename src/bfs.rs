use std::collections::VecDeque;
use Graph;
use Node;

pub fn breadth_first_search<'a, T: Graph<'a>>(graph: &'a T, start: Node) -> Vec<Option<Node>> {
    let mut q = VecDeque::new();
    let mut pred = vec![None; graph.num_nodes()];

    q.push_front(start);
    pred[start] = Some(start);

    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        for u in graph.neighbors(v) {
            if pred[*u] == None {
                pred[*u] = Some(v);
                q.push_front(*u);
            }
        }
    }

    pred
}

#[cfg(test)]
mod tests {
    use Graph;
    use EdgeList;
    use breadth_first_search;

    #[test]
    fn simple() {
        let mut graph = EdgeList::new();
        graph.add_edge(0,1);
        graph.add_edge(1,2);

        assert_eq!(breadth_first_search(&graph, 0), vec![Some(0), Some(0), Some(1)]);
    }

    #[test]
    fn complete_graph() {
        let mut graph = EdgeList::new();

        for u in 0..100 {
            for v in 0..100 {
                graph.add_edge(u,v);
            }
        }

        assert_eq!(breadth_first_search(&graph, 0), vec![Some(0); graph.num_nodes()]);
    }

    #[test]
    fn line_graph() {
        let mut graph = EdgeList::new();

        for u in 0..100 {
            graph.add_edge(u,u+1);
        }

        let mut pred = vec![None; graph.num_nodes()];
        pred[0] = Some(0);
        for u in 1..101 {
            pred[u] = Some(u-1);
        }

        assert_eq!(breadth_first_search(&graph, 0), pred);
    }
}
