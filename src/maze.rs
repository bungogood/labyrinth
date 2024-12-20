use std::{fs::File, path::Path};

use crate::{
    graph::{Direction, Graph},
    solver::search::Context,
};
use image::{Rgb, RgbImage};

const GIF_FRAME_DELAY: u16 = 100;

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
    pub fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn to_coord(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    pub fn draw(&mut self, idx: usize, color: Rgb<u8>) {
        let (x, y) = self.to_coord(idx);
        self.image.put_pixel(x as u32, y as u32, color);
    }

    pub fn draw_path(&mut self, path: &[usize], color: Rgb<u8>) {
        for &index in path {
            self.draw(index, color);
        }
    }

    pub fn full_path(&mut self, graph: &Graph, path: &[usize]) -> Vec<usize> {
        let mut prev = path[0];
        let mut full_path = vec![prev];

        for &vertex in path.iter().skip(1) {
            let dir = graph.get_edge(prev, vertex).unwrap().direction;
            let (x, y) = self.to_coord(prev);
            let target = self.to_coord(vertex);

            let mut next = match dir {
                Direction::Right => (x + 1, y),
                Direction::Left => (x - 1, y),
                Direction::Down => (x, y + 1),
                Direction::Up => (x, y - 1),
            };

            while next != target {
                full_path.push(self.to_index(next.0, next.1));
                match dir {
                    Direction::Right => next.0 += 1,
                    Direction::Left => next.0 -= 1,
                    Direction::Down => next.1 += 1,
                    Direction::Up => next.1 -= 1,
                }
            }
            full_path.push(vertex);
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
                start = Some(self.to_index(x, 0));
                prevline[x] = start.clone();
            }
        }

        // Second pass: Find all the path vertex and connect neighbors
        for y in 1..self.height - 1 {
            let mut prev = None;
            for x in 1..self.width - 1 {
                if self.is_path(x, y) {
                    if self.dir_change(x, y) {
                        let cur = self.to_index(x, y);

                        // Link with the previous vertex in the same row (left)
                        if let Some(prev_vertex) = prev {
                            let weight = cur - prev_vertex;
                            graph.add_edge(prev_vertex, cur, weight, Direction::Right);
                        }

                        // Link with the vertex in the previous row (up)
                        if let Some(prevline_vertex) = prevline[x] {
                            let (_, py) = self.to_coord(prevline_vertex);
                            let weight = y - py;
                            graph.add_edge(prevline_vertex, cur, weight, Direction::Down);
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
                end = Some(self.to_index(x, self.height - 1));
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

    pub fn png_explore(&mut self, graph: &Graph, context: &Context, outfile: impl AsRef<Path>) {
        let full_path = self.full_path(&graph, &context.path.as_ref().unwrap());

        self.draw_path(&context.visited, Rgb([0, 255, 0]));
        self.draw_path(&full_path, Rgb([255, 0, 0]));
        self.draw(context.start, Rgb([0, 0, 255]));
        self.draw(context.end, Rgb([0, 0, 255]));

        self.save(outfile).unwrap();
    }

    pub fn gif_explore(
        &mut self,
        graph: &Graph,
        context: &Context,
        dur_millis: u16,
        outfile: impl AsRef<Path>,
    ) {
        let num_frames = dur_millis / GIF_FRAME_DELAY;

        let items_per_frame = context.visited.len() as f32 / (num_frames - 1) as f32;

        let full_path = self.full_path(&graph, &context.path.as_ref().unwrap());

        // create a gif encoder
        // create a mutable buffer to store the image data
        let color_map = vec![255, 255, 255, 0, 0, 0, 0, 255, 0, 255, 0, 0];
        let mut image = File::create(outfile).unwrap();
        let mut encoder = gif::Encoder::new(
            &mut image,
            self.width as u16,
            self.height as u16,
            &color_map,
        )
        .unwrap();

        // encoder.set_repeat(gif::Repeat::Infinite).unwrap();

        let mut buf = vec![0; self.width * self.height];

        for x in 0..self.width {
            for y in 0..self.height {
                if !self.is_path(x, y) {
                    buf[self.to_index(x, y)] = 1;
                }
            }
        }

        let mut frame_counter = 0.0;
        for &index in context.visited.iter() {
            buf[index] = 2;
            frame_counter += 1.0;

            if frame_counter >= items_per_frame {
                frame_counter -= items_per_frame;
                let frame = gif::Frame {
                    width: self.width as u16,
                    height: self.height as u16,
                    buffer: std::borrow::Cow::Borrowed(buf.as_slice()),
                    ..Default::default()
                };
                encoder.write_frame(&frame).unwrap();
            }
        }

        for index in full_path {
            buf[index] = 3;
        }

        let frame = gif::Frame {
            width: self.width as u16,
            height: self.height as u16,
            buffer: std::borrow::Cow::Borrowed(buf.as_slice()),
            ..Default::default()
        };
        encoder.write_frame(&frame).unwrap();
    }
}
