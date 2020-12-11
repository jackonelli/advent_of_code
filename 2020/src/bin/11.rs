use std::fs::File;
use std::io::prelude::*;

type Grid = Vec<Vec<char>>;

// 2997 too high
fn main() {
    let file = "input/11/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data = contents.trim().lines();
    let grid = data
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Grid>();
    println!("Star 1: {}", run(grid.clone(), count_nb_adj, 4));
    println!("Star 2: {}", run(grid, count_nb_ray, 5));
}

fn run(grid: Grid, nb_counter: fn(&Grid, usize, usize) -> usize, occ_thresh: usize) -> usize {
    let mut grid = grid;
    let mut prev = 0;
    let mut curr = 1;
    while curr != prev {
        grid = tick(&grid, nb_counter, occ_thresh);
        prev = curr;
        curr = count_occ(&grid);
    }
    curr
}

fn tick(grid: &Grid, nb_counter: fn(&Grid, usize, usize) -> usize, occ_thresh: usize) -> Grid {
    let (height, width) = (grid.len(), grid[0].len());
    let mut new_grid = grid.to_vec();
    for y in 0..height {
        for x in 0..width {
            match grid[y][x] {
                '.' => {}
                'L' => {
                    let occ = nb_counter(&grid, x, y);
                    if occ == 0 {
                        new_grid[y][x] = '#';
                    }
                }
                '#' => {
                    let occ = nb_counter(&grid, x, y);
                    if occ >= occ_thresh {
                        new_grid[y][x] = 'L';
                    }
                }
                _ => panic!("inv tick char"),
            };
        }
    }
    new_grid
}

fn count_occ(grid: &Grid) -> usize {
    grid.iter()
        .map(|rows| rows.iter().filter(|c| **c == '#').count())
        .sum()
}

fn count_ray(grid: &Grid, ray: impl Iterator<Item = (usize, usize)>) -> usize {
    let seat = ray
        .map(|(x, y)| grid[y][x])
        .find(|c| *c == '#' || *c == 'L');
    match seat {
        Some(c) => {
            if c == '#' {
                1
            } else {
                0
            }
        }
        None => 0,
    }
}
fn count_nb_ray(grid: &Grid, x: usize, y: usize) -> usize {
    let (height, width) = (grid.len(), grid[0].len());
    let up_ray = (0..y).rev().map(|y| (x, y));
    let down_ray = (y + 1..height).map(|y| (x, y));
    let left_ray = (0..x).rev().map(|x| (x, y));
    let right_ray = (x + 1..width).map(|x| (x, y));
    let up_left_ray = (0..x).rev().zip((0..y).rev());
    let up_right_ray = (x + 1..width).zip((0..y).rev());
    let down_left_ray = (0..x).rev().zip(y + 1..height);
    let down_right_ray = (x + 1..width).zip(y + 1..height);

    let mut occ = 0;
    occ += count_ray(grid, up_ray);
    occ += count_ray(grid, down_ray);
    occ += count_ray(grid, left_ray);
    occ += count_ray(grid, right_ray);
    occ += count_ray(grid, up_left_ray);
    occ += count_ray(grid, up_right_ray);
    occ += count_ray(grid, down_left_ray);
    occ += count_ray(grid, down_right_ray);
    occ
}

fn count_nb_adj(grid: &Grid, x: usize, y: usize) -> usize {
    let (height, width) = (grid.len(), grid[0].len());
    let up = y.saturating_sub(1);
    let down = if y < height - 1 { y + 1 } else { y };
    let left = x.saturating_sub(1);
    let right = if x < width - 1 { x + 1 } else { x };
    let mut occ = 0;
    for j in up..down + 1 {
        for i in left..right + 1 {
            if i == x && j == y {
                continue;
            }
            match grid[j][i] {
                'L' => {} //empty += 1,
                '#' => occ += 1,
                '.' => {}
                _ => panic!("inv char"),
            }
        }
    }
    occ
}

fn print_grid(grid: &Grid) {
    for r in grid {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
    println!();
}
