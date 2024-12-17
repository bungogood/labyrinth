use clap::Parser;
use image::Rgb;
use labyrinth::dfs::dfs;
use labyrinth::dijkstra::dijkstra;
use labyrinth::maze::Maze;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Input file
    input: PathBuf,

    // Output file
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut maze = Maze::load(args.input)?;

    let (graph, start, end) = maze.parse().unwrap();

    let path = dijkstra(&graph, start, end).unwrap();

    let full_path = maze.full_path(&graph, &path);

    maze.draw_path(&full_path, Rgb([0, 255, 0]));
    maze.draw_path(&path, Rgb([255, 0, 0]));
    maze.draw(start, Rgb([0, 0, 255]));
    maze.draw(end, Rgb([0, 0, 255]));

    maze.save(args.output)?;

    println!("Path found: {:?}", path.len());

    // println!("Path found: {:?}", path);
    Ok(())
}
