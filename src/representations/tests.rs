use Generator;
use StaticGraph;
use Graph;
use Node;
use Edge;

use representations::AdjacencyList;
use representations::EdgeList;
use representations::Csr;

use generators::Erdos;


#[test]
fn creation_edgelist() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);

    assert!(graph.has_edge(0, 1));
    assert!(!graph.has_edge(1, 0));
    assert_eq!(graph.num_edges(), 1);
    assert_eq!(graph.num_nodes(), 2);
}

#[test]
fn creation_adjacencylist() {
    let mut graph = AdjacencyList::new();

    graph.add_edge(0,1);

    assert!(graph.has_edge(0, 1));
    assert!(!graph.has_edge(1, 0));
    assert_eq!(graph.num_edges(), 1);
    assert_eq!(graph.num_nodes(), 2);
}

fn creation_csr() {
    let mut graph = EdgeList::new();
    graph.add_edge(0,1);

    let graph = Csr::from_generator(&graph);

    assert!(graph.has_edge(0, 1));
    assert!(!graph.has_edge(1, 0));
    assert_eq!(graph.num_edges(), 1);
    assert_eq!(graph.num_nodes(), 2);
}

#[test]
fn add_edges_edgelist() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);
    graph.add_edge(2,5);
    graph.add_edge(0,3);

    assert!(graph.has_edge(2,5));
    assert!(!graph.has_edge(5,2));
    assert!(!graph.has_edge(3,2));
    assert_eq!(graph.num_edges(), 4);
    assert_eq!(graph.num_nodes(), 6);
}

#[test]
fn add_edges_adjacencylist() {
    let mut graph = AdjacencyList::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);
    graph.add_edge(2,5);
    graph.add_edge(0,3);

    assert!(graph.has_edge(2,5));
    assert!(!graph.has_edge(5,2));
    assert!(!graph.has_edge(3,2));
    assert_eq!(graph.num_edges(), 4);
    assert_eq!(graph.num_nodes(), 6);
}

fn add_edges_csr() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);
    graph.add_edge(2,5);
    graph.add_edge(0,3);

    let graph = Csr::from_generator(&graph);

    assert!(graph.has_edge(2,5));
    assert!(!graph.has_edge(5,2));
    assert!(!graph.has_edge(3,2));
    assert_eq!(graph.num_edges(), 4);
    assert_eq!(graph.num_nodes(), 6);
}


#[test]
fn add_many_edges_edgelist() {
    let mut graph = EdgeList::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    assert!(graph.has_edge(994,7));
    assert!(!graph.has_edge(994,13));
    assert_eq!(graph.num_edges(), 12000);
    assert_eq!(graph.num_nodes(), 1000);
}

#[test]
fn add_many_edges_adjacencylist() {
    let mut graph = AdjacencyList::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    assert!(graph.has_edge(994,7));
    assert!(!graph.has_edge(994,13));
    assert_eq!(graph.num_edges(), 12000);
    assert_eq!(graph.num_nodes(), 1000);
}

fn add_many_edges_csr() {
    let mut graph = EdgeList::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    let graph = Csr::from_generator(&graph);

    assert!(graph.has_edge(994,7));
    assert!(!graph.has_edge(994,12));
    assert_eq!(graph.num_edges(), 12000);
    assert_eq!(graph.num_nodes(), 1000);
}

#[test]
fn clear_graph_edgelist() {
    let mut graph = EdgeList::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    assert!(graph.has_edge(994,7));
    assert!(!graph.has_edge(994,13));
    assert_eq!(graph.num_edges(), 12000);
    assert_eq!(graph.num_nodes(), 1000);

    graph.clear();

    assert!(!graph.has_edge(994,7));
    assert_eq!(graph.num_edges(), 0);
    assert_eq!(graph.num_nodes(), 0);
}

#[test]
fn clear_graph_adjacencylist() {
    let mut graph = AdjacencyList::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    assert!(graph.has_edge(994,7));
    assert!(!graph.has_edge(994,13));
    assert_eq!(graph.num_edges(), 12000);
    assert_eq!(graph.num_nodes(), 1000);

    graph.clear();

    assert!(!graph.has_edge(994,7));
    assert_eq!(graph.num_edges(), 0);
    assert_eq!(graph.num_nodes(), 0);
}

fn clear_graph_csr() {
    let mut graph = EdgeList::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    let mut graph = Csr::from_generator(&graph);

    assert!(graph.has_edge(994,7));
    assert!(!graph.has_edge(994,12));
    assert_eq!(graph.num_edges(), 12000);
    assert_eq!(graph.num_nodes(), 1000);

    graph.clear();

    assert!(!graph.has_edge(994,7));
    assert_eq!(graph.num_edges(), 0);
    assert_eq!(graph.num_nodes(), 0);
}

#[test]
fn duplicate_edge_edgelist() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);

    assert!(graph.has_edge(0, 1));
    assert!(!graph.has_edge(1, 0));
    assert_eq!(graph.num_edges(), 4);
    assert_eq!(graph.num_nodes(), 2);
}

#[test]
fn duplicate_edge_adjacencylist() {
    let mut graph = AdjacencyList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);

    assert!(graph.has_edge(0, 1));
    assert!(!graph.has_edge(1, 0));
    assert_eq!(graph.num_edges(), 4);
    assert_eq!(graph.num_nodes(), 2);
}

fn duplicate_edge_csr() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);

    let graph = Csr::from_generator(&graph);

    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(1, 0));
    assert_eq!(graph.num_edges(), 4);
    assert_eq!(graph.num_nodes(), 2);
}

#[test]
fn reverse_edge_edgelist() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);

    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(1, 0));
    assert!(!graph.has_edge(0, 0));
    assert!(!graph.has_edge(1, 1));
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn reverse_edge_adjacencylist() {
    let mut graph = AdjacencyList::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);

    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(1, 0));
    assert!(!graph.has_edge(0, 0));
    assert!(!graph.has_edge(1, 1));
    assert_eq!(graph.num_edges(), 2);
}

fn reverse_edge_csr() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);

    let graph = Csr::from_generator(&graph);

    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(1, 0));
    assert!(!graph.has_edge(0, 0));
    assert!(!graph.has_edge(1, 1));
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn neighbors_edgelist() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,3]);
}

#[test]
fn neighbors_adjacencylist() {
    let mut graph = AdjacencyList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,3]);
}

fn neighbors_csr() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    let graph = Csr::from_generator(&graph);

    assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,3]);
}

#[test]
fn multi_neighbors_edgelist() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,2,3,3,3]);
    assert_eq!(graph.neighbors(1).collect::<Vec<Node>>(), vec![2]);
    assert_eq!(graph.neighbors(2).collect::<Vec<Node>>(), vec![]);
}

#[test]
fn multi_neighbors_adjacencylist() {
    let mut graph = AdjacencyList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,2,3,3,3]);
    assert_eq!(graph.neighbors(1).collect::<Vec<Node>>(), vec![2]);
    assert_eq!(graph.neighbors(2).collect::<Vec<Node>>(), vec![]);
}

fn multi_neighbors_csr() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    let graph = Csr::from_generator(&graph);

    assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,2,3,3,3]);
    assert_eq!(graph.neighbors(1).collect::<Vec<Node>>(), vec![2]);
    assert_eq!(graph.neighbors(2).collect::<Vec<Node>>(), vec![]);
}

#[test]
fn edges_edgelist() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    assert_eq!(graph.edges().collect::<Vec<Edge>>()[6], Edge::new(1,2));
    assert_eq!(graph.edges().collect::<Vec<Edge>>().len(), 7);
}

#[test]
fn edges_adjacencylist() {
    let mut graph = AdjacencyList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    assert_eq!(graph.edges().collect::<Vec<Edge>>()[6], Edge::new(1,2));
    assert_eq!(graph.edges().collect::<Vec<Edge>>().len(), 7);
}

fn edges_csr() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    let graph = Csr::from_generator(&graph);

    assert_eq!(graph.edges().collect::<Vec<Edge>>()[6], Edge::new(1,2));
    assert_eq!(graph.edges().collect::<Vec<Edge>>().len(), 7);
}

#[test]
fn to_edgelist_from_adjacecnylist() {
    let mut graph = AdjacencyList::new();

    for u in 0..100 {
        for v in 0..100 {
            graph.add_edge(u, v);
        }
    }

    let edges = graph.edges().collect::<Vec<Edge>>();
    let edgelist = EdgeList::from_generator(&graph);
    let edges_edgelist = edgelist.edges().collect::<Vec<Edge>>();

    assert_eq!(edges, edges_edgelist);
}

fn to_edgelist_from_csr() {
    let mut graph = EdgeList::new();

    for u in 0..100 {
        for v in 0..100 {
            graph.add_edge(u, v);
        }
    }

    let graph = Csr::from_generator(&graph);

    let edges = graph.edges().collect::<Vec<Edge>>();
    let edgelist = EdgeList::from_generator(&graph);
    let edges_edgelist = edgelist.edges().collect::<Vec<Edge>>();

    assert_eq!(edges, edges_edgelist);
}

#[test]
fn to_adjacencylist_from_edgelist() {
    let mut graph = EdgeList::new();

    for u in 0..100 {
        for v in 0..100 {
            graph.add_edge(u, v);
        }
    }

    let edges = graph.edges().collect::<Vec<Edge>>();
    let adjacencylist = AdjacencyList::from_generator(&graph);
    let edges_adjacencylist = adjacencylist.edges().collect::<Vec<Edge>>();

    assert_eq!(edges, edges_adjacencylist);
}

fn to_adjacencylist_csr() {
    let mut graph = EdgeList::new();

    for u in 0..100 {
        for v in 0..100 {
            graph.add_edge(u, v);
        }
    }

    let graph = Csr::from_generator(&graph);

    let edges = graph.edges().collect::<Vec<Edge>>();
    let adjacencylist = AdjacencyList::from_generator(&graph);
    let edges_adjacencylist = adjacencylist.edges().collect::<Vec<Edge>>();

    assert_eq!(edges, edges_adjacencylist);
}

#[test]
fn conversion_bfs() {
    let erdos = Erdos::new(1000, 0.01);
    let al = AdjacencyList::from_generator(&erdos);

    let pred_al = al.breadth_first_search(123);
    let el = EdgeList::from_generator(&al);
    let pred_el = el.breadth_first_search(123);

    assert_eq!(pred_al, pred_el);
}
