use std::ops::{Index, IndexMut};
use std::iter::{Iterator, repeat};

use graph::{Edge, Graph};

pub struct AdjacencyMatrix {
    rows: Vec<Vec<u32>>,
}

impl AdjacencyMatrix {
    fn new(size: usize) -> AdjacencyMatrix {
        let row = vec![0; size];
        let rows = repeat(row).take(size).collect();

        AdjacencyMatrix { rows: rows }
    }

    pub fn from_graph(g: &Graph) -> AdjacencyMatrix {
        let mut m = AdjacencyMatrix::new(g.vertex_count);

        for edge in &g.edges {
            m[edge.x][edge.y] = 1;
            if !g.directed {
                m[edge.y][edge.x] = 1;
            }
        }

        m
    }

    pub fn size(&self) -> usize {
        self.rows.len()
    }

    pub fn breadth_first_search(&self, start: usize) -> BreadthFirstSearch {
        BreadthFirstSearch::with_adjacency_matrix(self, start)
    }
}

// For convenience, enable writing matrix[x] instead of matrix.rows[x]).

impl Index<usize> for AdjacencyMatrix {
    type Output = Vec<u32>;

    fn index<'a>(&'a self, _index: usize) -> &'a Vec<u32> {
        &self.rows[_index]
    }
}

impl IndexMut<usize> for AdjacencyMatrix {
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut Vec<u32> {
        &mut self.rows[_index]
    }
}

// Breadth-first search.

struct BreadthFirstSearch<'a> {
    matrix: &'a AdjacencyMatrix,
    visited: Vec<bool>,
    queue: Vec<usize>,
    y: usize,
}

impl<'a> BreadthFirstSearch<'a> {
    fn with_adjacency_matrix(matrix: &AdjacencyMatrix, start: usize) -> BreadthFirstSearch {
        let mut visited = vec!(false; matrix.size());
        visited[start] = true;

        BreadthFirstSearch {
            matrix: matrix,
            visited: visited,
            queue: vec![start],
            y: 0
        }
    }
}

impl<'a> Iterator for BreadthFirstSearch<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        while !self.queue.is_empty() {
            let x = self.queue[0];

            for y in self.y..self.matrix.size() {
                if self.visited[y] || self.matrix[x][y] == 0 {
                    continue;
                }
                self.visited[y] = true;
                self.queue.push(y);
                self.y = y + 1;
                return Some(Edge { x: x, y: y });
            }

            // Process the next element in the queue.
            self.queue.remove(0);
            self.y = 0;
        }

        None
    }
}