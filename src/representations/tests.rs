use Graph;
use Node;
use Edge;

use representations::AdjacencyList;
use representations::EdgeList;

// AdjacencyList
#[test]
fn creation_adjacency_list() {
    creation::<AdjacencyList>();
}

#[test]
fn add_edges_adjacency_list() {
    add_edges::<AdjacencyList>();
}

#[test]
fn add_many_edges_adjacency_list() {
    add_many_edges::<AdjacencyList>();
}

#[test]
fn clear_graph_adjacency_list() {
    clear_graph::<AdjacencyList>();
}

#[test]
fn duplicate_edge_adjacency_list() {
    duplicate_edge::<AdjacencyList>();
}

#[test]
fn reverse_edge_adjacency_list() {
    reverse_edge::<AdjacencyList>();
}

#[test]
fn neighbors_adjacency_list() {
    neighbors::<AdjacencyList>();
}

#[test]
fn multi_neighbors_adjacency_list() {
    multi_neighbors::<AdjacencyList>();
}

#[test]
fn edges_adjacency_list() {
    edges::<AdjacencyList>();
}

// EdgeList
#[test]
fn creation_edge_list() {
    creation::<EdgeList>();
}

#[test]
fn add_edges_edge_list() {
    add_edges::<EdgeList>();
}

#[test]
fn add_many_edges_edge_list() {
    add_many_edges::<EdgeList>();
}

#[test]
fn clear_graph_edge_list() {
    clear_graph::<EdgeList>();
}

#[test]
fn duplicate_edge_edge_list() {
    duplicate_edge::<EdgeList>();
}

#[test]
fn reverse_edge_edge_list() {
    reverse_edge::<EdgeList>();
}

#[test]
fn neighbors_edge_list() {
    neighbors::<EdgeList>();
}

#[test]
fn multi_neighbors_edge_list() {
    multi_neighbors::<EdgeList>();
}

#[test]
fn edges_edge_list() {
    edges::<EdgeList>();
}

fn creation<T: Graph>() {
    let mut graph = T::new();

    graph.add_edge(0,1);

    assert!(graph.has_edge(0, 1));
    assert_eq!(graph.num_edges(), 1);
    assert_eq!(graph.num_nodes(), 2);
}

fn add_edges<T: Graph>() {
    let mut graph = T::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);
    graph.add_edge(2,5);
    graph.add_edge(0,3);

    assert!(graph.has_edge(2,5));
    assert!(!graph.has_edge(5,2));
    assert_eq!(graph.num_edges(), 4);
    assert_eq!(graph.num_nodes(), 6);
}

fn add_many_edges<T: Graph>() {
    let mut graph = T::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    assert!(graph.has_edge(994,7));
    assert!(!graph.has_edge(994,12));
    assert_eq!(graph.num_edges(), 12000);
    assert_eq!(graph.num_nodes(), 1000);
}

fn clear_graph<T: Graph>() {
    let mut graph = T::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    assert!(graph.has_edge(994,7));
    assert!(!graph.has_edge(994,12));
    assert_eq!(graph.num_edges(), 12000);
    assert_eq!(graph.num_nodes(), 1000);

    graph.clear();

    assert!(!graph.has_edge(994,7));
    assert_eq!(graph.num_edges(), 0);
    assert_eq!(graph.num_nodes(), 0);
}

fn duplicate_edge<T: Graph>() {
    let mut graph = T::new();

    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);

    assert!(graph.has_edge(0, 1));
    assert!(!graph.has_edge(1, 0));
    assert_eq!(graph.num_edges(), 4);
    assert_eq!(graph.num_nodes(), 2);
}

fn reverse_edge<T: Graph>() {
    let mut graph = T::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);

    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(1, 0));
    assert!(!graph.has_edge(0, 0));
    assert!(!graph.has_edge(1, 1));
    assert_eq!(graph.num_edges(), 2);
}

fn neighbors<T: Graph>() {
    let mut graph = T::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    assert_eq!(graph.neighbors(0).collect::<Vec<Node>>(), vec![1,2,3]);
}

fn multi_neighbors<T: Graph>() {
    let mut graph = T::new();

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

fn edges<T: Graph>() {
    let mut graph = T::new();

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
