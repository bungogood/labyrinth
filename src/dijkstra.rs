use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::graph::Graph;

#[derive(Debug, Eq, PartialEq)]
struct vertex {
    id: usize,
    dist: usize,
}

impl Ord for vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist) // Reverse order for min-heap
    }
}

impl PartialOrd for vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(graph: &Graph, start: usize, end: usize) -> Option<Vec<usize>> {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new(); // To reconstruct the path later
    let mut heap = BinaryHeap::new();

    // Initialize the distances to all vertex as infinity, except for the start vertex
    dist.insert(start, 0);
    heap.push(vertex { id: start, dist: 0 });

    while let Some(vertex {
        id,
        dist: curr_dist,
    }) = heap.pop()
    {
        // If we've reached the output vertex, stop early
        if id == end {
            break;
        }

        // Traverse neighbors
        if let Some(edges) = graph.get(&id) {
            for edge in edges {
                let next_dist = curr_dist + edge.weight;
                if next_dist < *dist.get(&edge.to).unwrap_or(&usize::MAX) {
                    dist.insert(edge.to, next_dist);
                    prev.insert(edge.to, id); // Track the path
                    heap.push(vertex {
                        id: edge.to,
                        dist: next_dist,
                    });
                }
            }
        }
    }

    // If the end vertex was reached, reconstruct the shortest path
    if dist.contains_key(&end) {
        let mut path = Vec::new();
        let mut current_vertex = end;

        // Reconstruct the path by following the previous vertex
        while let Some(&prev_vertex) = prev.get(&current_vertex) {
            path.push(current_vertex);
            current_vertex = prev_vertex;
        }
        path.push(start);
        path.reverse(); // Reverse to get the correct order from start to end

        Some(path)
    } else {
        None // No path found
    }
}
