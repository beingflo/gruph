use Generator;
use StaticGraph;
use Node;
use Edge;

use representations::EdgeList;

#[derive(Clone, Debug)]
pub struct Csr {
    col: Vec<Node>,
    row: Vec<usize>,
    num_edges: usize,
}

struct CsrIterator<'a> {
    csr: &'a Csr,
    idx: usize,
    u: usize,
}

impl<'a> Iterator for CsrIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.csr.num_edges() {
            None
        } else {
            let ret = Some(Edge::new(self.u, self.csr.col[self.idx])); 
            if self.idx + 1 >= self.csr.row[self.u+1] {
                self.u += 1;
            }
            self.idx += 1;

            ret
        }
    }
}

impl Generator for Csr {
    fn edges<'a>(&'a self) -> Box<Iterator<Item=Edge> + 'a> {
        Box::new(CsrIterator { csr: self, u: 0, idx: 0 })
    }
}

impl StaticGraph for Csr {
    fn from_generator<T: Generator>(gen: &T) -> Self {
        let el = EdgeList::from_generator(gen);

        let mut deg = vec![0; el.num_nodes()];

        for e in el.edges() {
            deg[e.u()] += 1;
        }

        let mut col = vec![0; el.num_edges()];
        let mut row = vec![0; el.num_nodes() + 1];

        let mut cumul = 0;
        for i in 0..el.num_nodes() {
            row[i] = cumul;
            cumul += deg[i];
        }

        row[el.num_nodes()] = el.num_edges();

        let mut idx = vec![0; el.num_nodes()];

        for e in el.edges() {
            col[row[e.u()] + idx[e.u()]] = e.v();
            idx[e.u()] += 1;
        }

        Csr { col, row, num_edges: el.num_edges() }
    }

    fn num_nodes(&self) -> usize {
        if self.row.len() == 0 {
            0
        } else {
            self.row.len() - 1
        }
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }

    fn has_edge(&self, from: Node, to: Node) -> bool {
        if self.row.len() <= from {
            return false;
        }

        for i in self.row[from]..self.row[from+1] {
            if self.col[i] == to {
                return true;
            }
        }
        false
    }

    fn neighbors<'a>(&'a self, vertex: Node) -> Box<Iterator<Item=Node> + 'a> {
        let num_neighbors = self.row[vertex+1] - self.row[vertex];
        Box::new(self.col.iter().skip(self.row[vertex]).take(num_neighbors).map(|&u| u))
    }

    fn clear(&mut self) {
        self.col.clear();
        self.row.clear();
        self.num_edges = 0;
    }
}
