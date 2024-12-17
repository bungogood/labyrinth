use std::collections::{HashMap, HashSet};

use crate::graph::Graph;

pub fn reconstruct_path(
    prev: &HashMap<usize, usize>,
    start: usize,
    end: usize,
) -> Option<Vec<usize>> {
    if prev.contains_key(&end) {
        let mut path = Vec::new();
        let mut current_vertex = end;

        // Reconstruct the path by following the previous vertex
        while let Some(&prev_vertex) = prev.get(&current_vertex) {
            path.push(current_vertex);
            current_vertex = prev_vertex;
        }
        path.push(start); // Add the start vertex
        path.reverse(); // Reverse to get the correct order from start to end

        Some(path)
    } else {
        None // No path found
    }
}

pub fn dfs(graph: &Graph, start: usize, end: usize) -> Option<Vec<usize>> {
    let mut visited = HashSet::new(); // Set to keep track of visited vertex
    let mut prev = HashMap::new(); // To reconstruct the path later
    let mut stack = vec![start]; // Stack to simulate DFS

    // Depth-First Search
    while let Some(current_vertex) = stack.pop() {
        // If we reached the end vertex, stop and reconstruct the path
        if current_vertex == end {
            break;
        }

        // Mark the current vertex as visited
        if !visited.contains(&current_vertex) {
            visited.insert(current_vertex);

            // Traverse neighbors and add them to the stack
            if let Some(neighbors) = graph.get_neighbors(&current_vertex) {
                for neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                        prev.insert(neighbor, current_vertex); // Track the path
                    }
                }
            }
        }
    }

    // Reconstruct the path if the end vertex was reached
    reconstruct_path(&prev, start, end)
}
