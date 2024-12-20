use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::graph::Graph;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    id: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Min-heap (reverse order for BinaryHeap)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn reconstruct(prev: &HashMap<usize, usize>, start: usize, end: usize) -> Option<Vec<usize>> {
    if prev.contains_key(&end) {
        let mut path = Vec::new();
        let mut current = end;
        while let Some(&prev) = prev.get(&current) {
            path.push(current);
            current = prev;
        }
        path.push(start);
        path.reverse();
        Some(path)
    } else {
        None
    }
}

pub fn search<F>(graph: &Graph, start: usize, end: usize, heuristic: F) -> Context
where
    F: Fn(usize, usize, usize) -> usize, // Heuristic function
{
    let mut context = Context::search(start, end);

    let mut queue = BinaryHeap::new(); // Priority queue for A*
    let mut prev = HashMap::new(); // Tracks the path
    let mut dist = HashMap::new(); // Cost from start to each node

    // Initialize the start node
    dist.insert(start, 0);
    queue.push(Node {
        id: start,
        cost: heuristic(start, dist[&start], end),
    });

    while let Some(Node { id: current, .. }) = queue.pop() {
        // If we reached the end, reconstruct the path
        if current == end {
            break;
        }

        context.visit(current);

        // Explore neighbors
        if let Some(edges) = graph.get(&current) {
            let cur_dist = dist[&current];
            for edge in edges {
                let total_dist = cur_dist + edge.weight;
                if total_dist < *dist.get(&edge.to).unwrap_or(&usize::MAX) {
                    prev.insert(edge.to, current);
                    dist.insert(edge.to, total_dist);
                    queue.push(Node {
                        id: edge.to,
                        cost: heuristic(edge.to, dist[&edge.to], end),
                    });
                }
            }
        }
    }

    if let Some(path) = reconstruct(&prev, start, end) {
        context.set_path(path);
    }

    context
}

pub struct Context {
    pub start: usize,
    pub end: usize,
    pub path: Option<Vec<usize>>,
    pub visited: Vec<usize>,
}

impl Context {
    pub fn search(start: usize, end: usize) -> Self {
        Context {
            start,
            end,
            visited: vec![],
            path: None,
        }
    }

    pub fn visit(&mut self, vertex: usize) {
        self.visited.push(vertex);
    }

    pub fn set_path(&mut self, path: Vec<usize>) {
        self.path = Some(path);
    }
}
