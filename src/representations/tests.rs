use Generator;
use StaticGraph;
use Graph;
use Node;
use Edge;

use representations::AdjacencyList;
use representations::EdgeList;
use representations::Csr;

use generators::Erdos;

// AdjacencyList
#[test]
fn creation_adjacencylist() {
    creation::<AdjacencyList>();
}

#[test]
fn add_edges_adjacencylist() {
    add_edges::<AdjacencyList>();
}

#[test]
fn add_many_edges_adjacencylist() {
    add_many_edges::<AdjacencyList>();
}

#[test]
fn clear_graph_adjacencylist() {
    clear_graph::<AdjacencyList>();
}

#[test]
fn duplicate_edge_adjacencylist() {
    duplicate_edge::<AdjacencyList>();
}

#[test]
fn reverse_edge_adjacencylist() {
    reverse_edge::<AdjacencyList>();
}

#[test]
fn neighbors_adjacencylist() {
    neighbors::<AdjacencyList>();
}

#[test]
fn multi_neighbors_adjacencylist() {
    multi_neighbors::<AdjacencyList>();
}

#[test]
fn edges_adjacencylist() {
    edges::<AdjacencyList>();
}

#[test]
fn to_edgelist_adjacencylist() {
    to_edgelist::<AdjacencyList>();
}

#[test]
fn to_adjacencylist_adjacencylist() {
    to_adjacencylist::<AdjacencyList>();
}

// EdgeList
#[test]
fn creation_edgelist() {
    creation::<EdgeList>();
}

#[test]
fn add_edges_edgelist() {
    add_edges::<EdgeList>();
}

#[test]
fn add_many_edges_edgelist() {
    add_many_edges::<EdgeList>();
}

#[test]
fn clear_graph_edgelist() {
    clear_graph::<EdgeList>();
}

#[test]
fn duplicate_edge_edgelist() {
    duplicate_edge::<EdgeList>();
}

#[test]
fn reverse_edge_edgelist() {
    reverse_edge::<EdgeList>();
}

#[test]
fn neighbors_edgelist() {
    neighbors::<EdgeList>();
}

#[test]
fn multi_neighbors_edgelist() {
    multi_neighbors::<EdgeList>();
}

#[test]
fn edges_edgelist() {
    edges::<EdgeList>();
}

#[test]
fn to_edgelist_edgelist() {
    to_edgelist::<EdgeList>();
}

#[test]
fn to_adjacencylist_edgelist() {
    to_adjacencylist::<EdgeList>();
}

// CSR
#[test]
fn creation_csr() {
    creation_static::<Csr>();
}

#[test]
fn add_edges_csr() {
    add_edges_static::<Csr>();
}

#[test]
fn add_many_edges_csr() {
    add_many_edges_static::<Csr>();
}

#[test]
fn clear_graph_csr() {
    clear_graph_static::<Csr>();
}

#[test]
fn duplicate_edge_csr() {
    duplicate_edge_static::<Csr>();
}

#[test]
fn reverse_edge_csr() {
    reverse_edge_static::<Csr>();
}

#[test]
fn neighbors_csr() {
    neighbors_static::<Csr>();
}

#[test]
fn multi_neighbors_csr() {
    multi_neighbors_static::<Csr>();
}

#[test]
fn edges_csr() {
    edges_static::<Csr>();
}

#[test]
fn to_edgelist_csr() {
    to_edgelist_static::<Csr>();
}

#[test]
fn to_adjacencylist_csr() {
    to_adjacencylist_static::<Csr>();
}

fn creation<T: Graph>() {
    let mut graph = T::new();

    graph.add_edge(0,1);

    assert!(graph.has_edge(0, 1));
    assert_eq!(graph.num_edges(), 1);
    assert_eq!(graph.num_nodes(), 2);
}

fn creation_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();
    graph.add_edge(0,1);

    let graph = T::from_generator(&graph);

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

fn add_edges_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);
    graph.add_edge(2,5);
    graph.add_edge(0,3);

    let graph = T::from_generator(&graph);

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

fn add_many_edges_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    let graph = T::from_generator(&graph);

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

fn clear_graph_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    for u in 0..1000 {
        for v in 0..12 {
            graph.add_edge(u,v);
        }
    }

    let mut graph = T::from_generator(&graph);

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

fn duplicate_edge_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);
    graph.add_edge(0,1);

    let graph = T::from_generator(&graph);

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

fn reverse_edge_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(1,0);

    let graph = T::from_generator(&graph);

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

fn neighbors_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    let graph = T::from_generator(&graph);

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

fn multi_neighbors_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    let graph = T::from_generator(&graph);

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

fn edges_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(0,2);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(0,3);
    graph.add_edge(1,2);

    let graph = T::from_generator(&graph);

    assert_eq!(graph.edges().collect::<Vec<Edge>>()[6], Edge::new(1,2));
    assert_eq!(graph.edges().collect::<Vec<Edge>>().len(), 7);
}

fn to_edgelist<T: Graph>() {
    let mut graph = T::new();

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

fn to_edgelist_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    for u in 0..100 {
        for v in 0..100 {
            graph.add_edge(u, v);
        }
    }

    let graph = T::from_generator(&graph);

    let edges = graph.edges().collect::<Vec<Edge>>();
    let edgelist = EdgeList::from_generator(&graph);
    let edges_edgelist = edgelist.edges().collect::<Vec<Edge>>();

    assert_eq!(edges, edges_edgelist);
}

fn to_adjacencylist<T: Graph>() {
    let mut graph = T::new();

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

fn to_adjacencylist_static<T: StaticGraph>() {
    let mut graph = EdgeList::new();

    for u in 0..100 {
        for v in 0..100 {
            graph.add_edge(u, v);
        }
    }

    let graph = T::from_generator(&graph);

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
