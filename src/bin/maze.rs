use clap::Parser;
use image::Rgb;
use labyrinth::maze::Maze;
use labyrinth::solver::{
    heuristic::{astar_heuristic, dfs_heuristic, dijkstra_heuristic, greedy_heuristic},
    search,
};
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Input file
    input: PathBuf,

    // Output file
    output: PathBuf,

    // Animation
    #[clap(short, long)]
    animation: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut maze = Maze::load(args.input)?;

    let (graph, start, end) = maze.parse().unwrap();

    let context = search(&graph, start, end, dfs_heuristic);
    // let context = search(&graph, start, end, dijkstra_heuristic);
    // let context = search(&graph, start, end, greedy_heuristic(&maze));
    // let context = search(&graph, start, end, astar_heuristic(&maze));

    if args.animation {
        maze.gif_explore(&graph, &context, 5000, &args.output);
    } else {
        maze.png_explore(&graph, &context, &args.output);
    }

    Ok(())
}
