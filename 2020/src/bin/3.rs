use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/3/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data = contents.trim().lines();
    let grid = data
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("Star 1: {}", count_trees(&grid, (3, 1)));
    let steps = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "Star 2: {}",
        steps
            .iter()
            .map(|s| count_trees(&grid, *s))
            .product::<usize>()
    );
}

fn count_trees(grid: &[Vec<char>], step: (usize, usize)) -> usize {
    let (x_step, y_step) = step;
    let (width, height) = (grid[0].len(), grid.len());
    let x_idx = (0..width).cycle().step_by(x_step);
    let y_idx = (0..height).step_by(y_step);

    y_idx
        .zip(x_idx)
        .map(|(y, x)| grid[y][x])
        .filter(|c| *c == '#')
        .count()
}
