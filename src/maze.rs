use std::path::Path;

use crate::graph::{Direction, Graph};
use image::{Rgb, RgbImage};

// Maze struct that holds the width, height, and image data
pub struct Maze {
    width: usize,
    height: usize,
    image: RgbImage,
}

impl Maze {
    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let img =
            image::open(path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(Maze::new(img.to_rgb8()))
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        self.image
            .save(path)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn new(image: RgbImage) -> Self {
        let (width, height) = image.dimensions();
        Maze {
            width: width as usize,
            height: height as usize,
            image,
        }
    }

    // Check if the given (y, x) is a path (not black pixel)
    fn is_path(&self, x: usize, y: usize) -> bool {
        let pixel = self.image.get_pixel(x as u32, y as u32);
        pixel.0[0] != 0 && pixel.0[1] != 0 && pixel.0[2] != 0
    }

    // Convert (y, x) to a unique index in the graph
    fn to_index(&self, y: usize, x: usize) -> usize {
        y * self.width + x
    }

    fn to_row_col(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    pub fn draw(&mut self, idx: usize, color: Rgb<u8>) {
        let (y, x) = self.to_row_col(idx);
        self.image.put_pixel(x as u32, y as u32, color);
    }

    pub fn draw_path(&mut self, path: &[usize], color: Rgb<u8>) {
        for &index in path {
            self.draw(index, color);
        }
    }

    pub fn full_path(&mut self, graph: &Graph, path: &[usize]) -> Vec<usize> {
        let mut full_path = vec![path[0]];
        let mut prev = path[0];

        for &vertex in path.iter().skip(1) {
            let dir = graph.get_edge(prev, vertex).unwrap().direction;
            let (r1, c1) = self.to_row_col(prev);
            let target = self.to_row_col(vertex);

            let mut next = match dir {
                Direction::Right => (r1, c1 + 1),
                Direction::Left => (r1, c1 - 1),
                Direction::Down => (r1 + 1, c1),
                Direction::Up => (r1 - 1, c1),
            };

            while next != target {
                full_path.push(self.to_index(next.0, next.1));
                match dir {
                    Direction::Right => next.1 += 1,
                    Direction::Left => next.1 -= 1,
                    Direction::Down => next.0 += 1,
                    Direction::Up => next.0 -= 1,
                }
            }
            prev = vertex;
        }

        full_path
    }

    fn dir_change(&self, x: usize, y: usize) -> bool {
        let up = self.is_path(x, y - 1);
        let down = self.is_path(x, y + 1);
        let left = self.is_path(x - 1, y);
        let right = self.is_path(x + 1, y);

        up != down || left != right || (up && down && left && right)
    }

    // Parse the maze image into a graph representation
    pub fn parse(&self) -> Option<(Graph, usize, usize)> {
        let mut graph = Graph::new();
        let mut start = None;
        let mut end = None;
        let mut prevline = vec![None; self.width];

        // First pass: Find the start vertex in the first row
        for x in 1..self.width - 1 {
            if self.is_path(x, 0) {
                start = Some(self.to_index(0, x));
                prevline[x] = start.clone();
            }
        }

        // Second pass: Find all the path vertex and connect neighbors
        for y in 1..self.height - 1 {
            let mut prev = None;
            for x in 1..self.width - 1 {
                if self.is_path(x, y) {
                    if self.dir_change(x, y) {
                        let cur = self.to_index(y, x);

                        // Link with the previous vertex in the same row (left)
                        if let Some(prev_vertex) = prev {
                            graph.add_edge(prev_vertex, cur, 1, Direction::Right);
                        }

                        // Link with the vertex in the previous row (up)
                        if let Some(prevline_vertex) = prevline[x] {
                            graph.add_edge(prevline_vertex, cur, 1, Direction::Down);
                        }

                        prev = Some(cur);
                        prevline[x] = Some(cur);
                    }
                } else {
                    prev = None;
                    prevline[x] = None;
                }
            }
        }

        // Final pass: Find the end vertex in the last row and link to the previous row
        for x in 1..self.width - 1 {
            if self.is_path(x, self.height - 1) {
                end = Some(self.to_index(self.height - 1, x));
                if let Some(prevline_vertex) = prevline[x] {
                    graph.add_edge(prevline_vertex, end.unwrap(), 1, Direction::Down);
                }
                break;
            }
        }

        // If both start and end are found, return the graph along with start and end indices
        if let (Some(start), Some(end)) = (start, end) {
            Some((graph, start, end))
        } else {
            None // If no valid start or end found
        }
    }
}
