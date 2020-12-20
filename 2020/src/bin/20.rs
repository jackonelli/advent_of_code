use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::Split;

type Tiles = Vec<Tile>;
type Pat = Vec<char>;

fn main() {
    let file = "input/20/input";
    //let file = "input/19/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");

    let data = contents.trim().split("\n\n");
    let tiles = parse_tiles(data.clone());
    let (corners, edges): (Vec<usize>, Vec<usize>) = find_corners(tiles.clone());
    println!("corners: {}, edges: {}", &corners.len(), &edges.len());
    //println!("Star 1: {}", corners.iter().product::<usize>());
    let puzzle = solve_puzzle(&tiles, &corners);
}

fn solve_puzzle(tiles: &[Tile], corners: &[usize]) -> Vec<Pat> {
    let mut puzzle = init_puzzle(&tiles);
    println!("{} {}", &puzzle.len(), &puzzle[0].len());
    puzzle
}

fn init_puzzle(tiles: &[Tile]) -> Vec<Pat> {
    let num_tiles = tiles.len();
    let tile_size = tiles[0].pat.len();
    let mut puzzle = Vec::new();
    for _ in 0..(num_tiles * tile_size) {
        puzzle.push(vec!['.'; num_tiles * tile_size]);
    }
    puzzle
}
fn parse_tiles(input: Split<&str>) -> Vec<Tile> {
    let re = Regex::new(r"^Tile (\d+):$").unwrap();
    let mut tiles = Vec::new();
    for tile in input {
        let mut tile = tile.lines();
        let id: usize = re
            .captures(tile.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let pat = tile.map(|s| s.chars().collect()).collect();
        tiles.push(Tile { id, pat })
    }
    tiles
}

fn find_corners(tiles: Tiles) -> (Vec<usize>, Vec<usize>) {
    let mut edges = HashMap::new();
    for tile in &tiles {
        for edge in tile.edge_perms() {
            let entry = edges.entry(edge).or_insert(0);
            *entry += 1;
        }
    }
    //for (e, id) in &edges {
    //    println!("{}: {:?}", e.iter().collect::<String>(), id);
    //}

    let unique_count = tiles.iter().map(|t| {
        (
            t.id,
            t.edge_perms()
                .into_iter()
                .filter(|edge| *edges.get(edge).unwrap() == 1)
                .count(),
        )
    });
    (
        unique_count
            .clone()
            .filter(|(_id, count)| *count == 4)
            .map(|(id, _count)| id)
            .collect(),
        unique_count
            .clone()
            .filter(|(_id, count)| *count == 2)
            .map(|(id, _count)| id)
            .collect(),
    )
}

fn star_2(mut data: Split<&str>) -> usize {
    1
}

fn parse_rules(rules: &str) -> Tiles {
    let rules = rules.trim().lines();
    let mut parsed_rules = Tiles::new();

    for line in rules {}
    parsed_rules
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    pat: Vec<Pat>,
}

impl Tile {
    fn edge_perms(&self) -> Vec<Pat> {
        let size = self.pat.len();

        let top = self.pat[0].clone();
        let bot = self.pat[size - 1].clone();
        let left = self.pat.iter().map(|row| row[0]).collect::<Pat>();
        let right = self.pat.iter().map(|row| row[size - 1]).collect::<Pat>();
        vec![
            top.clone(),
            bot.clone(),
            left.clone(),
            right.clone(),
            top.into_iter().rev().collect(),
            bot.into_iter().rev().collect(),
            left.into_iter().rev().collect(),
            right.into_iter().rev().collect(),
        ]
    }
    fn print(&self) {
        for row in &self.pat {
            for col in row {
                print!("{}", col)
            }
            println!();
        }
    }
}
