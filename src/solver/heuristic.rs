use crate::maze::Maze;

pub fn dfs_heuristic(_node: usize, _dist: usize, _target: usize) -> usize {
    0
}

pub fn dijkstra_heuristic(_node: usize, dist: usize, _target: usize) -> usize {
    dist
}

pub fn greedy_heuristic(maze: &Maze) -> impl Fn(usize, usize, usize) -> usize + '_ {
    move |node, _, target| {
        let (nx, ny) = maze.to_coord(node);
        let (tx, ty) = maze.to_coord(target);
        ((nx as isize - tx as isize).abs() + (ny as isize - ty as isize).abs()) as usize
    }
}

pub fn astar_heuristic(maze: &Maze) -> impl Fn(usize, usize, usize) -> usize + '_ {
    move |node, dist, target| {
        let (nx, ny) = maze.to_coord(node);
        let (tx, ty) = maze.to_coord(target);
        let man = ((nx as isize - tx as isize).abs() + (ny as isize - ty as isize).abs()) as usize;
        man + dist
    }
}
