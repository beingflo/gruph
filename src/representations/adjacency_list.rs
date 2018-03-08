use Generator;
use StaticGraph;
use Graph;
use Node;
use Edge;

use std::cmp;
use std::iter;

#[derive(Clone, Debug)]
pub struct AdjacencyList {
    adj: Vec<Vec<Node>>,
    num_nodes: usize,
}

impl Generator for AdjacencyList {
    fn edges<'a>(&'a self) -> Box<Iterator<Item=Edge> + 'a> {
        Box::new(self.adj.iter().enumerate().flat_map(|(u, vec)| vec.iter().map(move |v| Edge::new(u, *v))))
    }
}

impl StaticGraph for AdjacencyList {
    fn from_generator<T: Generator>(gen: &T) -> Self {
        let mut al = AdjacencyList::new();

        for e in gen.edges() {
            al.add_edge(e.u(), e.v());
        }

        al
    }

    fn num_nodes(&self) -> usize {
        if self.adj.len() == 0 {
            0
        } else {
            self.num_nodes + 1
        }
    }

    fn num_edges(&self) -> usize {
        let mut num_edges = 0;
        for v in &self.adj {
            num_edges += v.len();
        }

        num_edges
    }

    fn has_edge(&self, from: Node, to: Node) -> bool {
        if self.adj.len() <= from {
            return false;
        }

        for u in &self.adj[from] {
            if *u == to {
                return true;
            }
        }

        false
    }

    fn neighbors<'a>(&'a self, vertex: Node) -> Box<Iterator<Item=Node> + 'a> {
        if vertex >= self.adj.len() {
            Box::new(iter::empty())
        } else {
            Box::new(self.adj[vertex].iter().map(|&v| v))
        }
    }

    fn clear(&mut self) {
        self.adj.clear();
        self.num_nodes = 0;
    }
}

impl Graph for AdjacencyList {
    fn new() -> Self {
        AdjacencyList { adj: vec![], num_nodes: 0 }
    }

    fn add_edge(&mut self, from: Node, to: Node) {
        if self.adj.len() <= from {
            while self.adj.len() <= from {
                self.adj.push(vec![]);
            }
        }

        self.adj[from].push(to);

        self.num_nodes = cmp::max(self.num_nodes, from);
        self.num_nodes = cmp::max(self.num_nodes, to);
    }
}
