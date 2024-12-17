use std::collections::HashMap;

pub struct Graph {
    vertex_count: usize,
    edge_count: usize,
    graph: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertex_count: 0,
            edge_count: 0,
            graph: HashMap::new(),
        }
    }

    pub fn add_directed_edge(
        &mut self,
        from: usize,
        to: usize,
        weight: usize,
        direction: Direction,
    ) {
        if !self.graph.contains_key(&from) {
            self.vertex_count += 1; // this is not correct for a directed graph
        }
        self.edge_count += 1;
        self.graph.entry(from).or_insert(vec![]).push(Edge {
            to,
            weight,
            direction,
        });
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: usize, direction: Direction) {
        self.add_directed_edge(from, to, weight, direction);
        self.add_directed_edge(to, from, weight, direction.opposite());
    }

    pub fn get(&self, key: &usize) -> Option<&Vec<Edge>> {
        self.graph.get(key)
    }

    pub fn get_neighbors(&self, key: &usize) -> Option<Vec<usize>> {
        self.graph
            .get(key)
            .map(|edges| edges.iter().map(|edge| edge.to).collect())
    }

    pub fn get_edge(&self, from: usize, to: usize) -> Option<&Edge> {
        self.graph
            .get(&from)
            .and_then(|edges| edges.iter().find(|edge| edge.to == to))
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}

impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // sort keys in order then output "{}: [({}, {}), ...]"
        let mut vertices: Vec<usize> = self.graph.keys().copied().collect();
        vertices.sort_unstable();
        for vertex in vertices {
            let mut edges = self.graph[&vertex].clone();
            edges.sort_unstable_by_key(|edge| edge.to);

            // do not use .join() to avoid trailing comma
            let edge_str = edges
                .iter()
                .map(|edge| format!("({}, {})", edge.to, edge.weight))
                .collect::<Vec<String>>()
                .join(", ");

            writeln!(f, "{}: [{}]", vertex, edge_str)?;
        }
        Ok(())
    }
}

impl std::ops::Index<&usize> for Graph {
    type Output = Vec<Edge>;

    fn index(&self, index: &usize) -> &Self::Output {
        &self.graph[index]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Edge {
    pub to: usize,
    pub weight: usize,
    pub direction: Direction,
}
