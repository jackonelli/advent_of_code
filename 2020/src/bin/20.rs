use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::Split;

const NESSIE: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

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
    let (corners, _, _) = find_edge_tiles(&tiles);
    println!(
        "Star 1: ({}) = {}",
        corners
            .iter()
            .map(|id| format!("{}", id))
            .collect::<Vec<String>>()
            .join(" * "),
        corners.clone().iter().product::<usize>()
    );
    let puzzle = solve_puzzle(&tiles);
    let monster_pat = Monster {
        pat: NESSIE.lines().map(|l| l.chars().collect::<Pat>()).collect(),
    };
    let trimmed_puzzle = remove_borders(puzzle, tiles[0].size());
    let highlight = monster_count(trimmed_puzzle, monster_pat);
    //print_puzzle(&highlight);
    println!(
        "Star 2: {}",
        highlight.iter().flatten().filter(|c| **c == '#').count()
    )
}

fn align_monster(puzzle: &[Pat], monster: Monster) -> Monster {
    let puzzle_size = puzzle.len();
    let mut tmp = monster;
    for _ in 0..4 {
        tmp = tmp.rot_90();
        let (height, width) = tmp.size();
        for row in 0..puzzle_size - height {
            for col in 0..puzzle_size - width {
                if comp_patch(&puzzle, &tmp, row, col) {
                    return tmp;
                }
            }
        }
    }
    tmp = tmp.flip();
    for _ in 0..4 {
        tmp = tmp.rot_90();
        let (height, width) = tmp.size();
        for row in 0..puzzle_size - height {
            for col in 0..puzzle_size - width {
                if comp_patch(&puzzle, &tmp, row, col) {
                    return tmp;
                }
            }
        }
    }
    panic!("No match");
}

fn monster_count(puzzle: Vec<Pat>, monster: Monster) -> Vec<Pat> {
    let monster = align_monster(&puzzle, monster);
    let mut puzzle = puzzle;
    let puzzle_size = puzzle.len();
    let (height, width) = monster.size();
    for row in 0..puzzle_size - height {
        for col in 0..puzzle_size - width {
            if comp_patch(&puzzle, &monster, row, col) {
                puzzle = highlight_monster(puzzle, &monster, row, col);
            }
        }
    }
    puzzle
}

fn highlight_monster(puzzle: Vec<Pat>, monster: &Monster, row: usize, col: usize) -> Vec<Pat> {
    let mut puzzle = puzzle;
    let (height, width) = monster.size();
    for i in 0..height {
        for j in 0..width {
            let c_m = monster.pat[i][j];
            if c_m == '#' {
                puzzle[row + i][col + j] = 'O';
            }
        }
    }
    puzzle
}

fn comp_patch(puzzle: &[Pat], monster: &Monster, row: usize, col: usize) -> bool {
    let (height, width) = monster.size();
    for i in 0..height {
        for j in 0..width {
            let c_p = puzzle[row + i][col + j];
            let c_m = monster.pat[i][j];
            //print!("{}", c_m);
            if c_m == '#' && c_p == '.' {
                return false;
            }
        }
    }
    true
}

// TODO: arb. starting piece.
fn solve_puzzle(tiles: &[Tile]) -> Vec<Pat> {
    let num_tiles = (tiles.len() as f32).sqrt() as usize;
    let mut puzzle = init_puzzle(&tiles);

    let (corners, _, edge_map) = find_edge_tiles(&tiles);
    let top_left = tiles.iter().find(|t| t.id == corners[0]).expect("no tile");

    let border_pats: Vec<Pat> = top_left
        .edge_perms()
        .iter()
        .filter(|edge| edge_map.get(*edge).unwrap().len() == 1)
        .cloned()
        .collect();
    let (top_pat, left_pat) = (border_pats[0].clone(), border_pats[1].clone());
    let left = top_left;

    let mut left = left.clone().align_left(left_pat.clone());
    if left.top() != top_pat {
        left = left.flip();
        left = left.clone().align_left(left_pat);
    }
    let mut row = get_row(left.clone(), &tiles, &edge_map);
    lay_row(0, &row, &mut puzzle);
    for i in 1..num_tiles {
        let cands: Vec<Tile> = edge_map
            .get(&left.bot())
            .unwrap()
            .iter()
            .filter(|id| **id != left.id)
            .map(|id| tiles.iter().find(|t| t.id == *id).unwrap())
            .cloned()
            .collect();
        left = cands[0].clone().align_top(left.bot());
        row = get_row(left.clone(), &tiles, &edge_map);
        lay_row(i, &row, &mut puzzle);
    }
    puzzle
}

fn lay_row(row_num: usize, row: &[Tile], puzzle: &mut Vec<Pat>) {
    let tile_size = row[0].size();
    for sub_row in 0..tile_size {
        puzzle[row_num * tile_size + sub_row] = row
            .iter()
            .map(|t| t.pat[sub_row].clone())
            .flatten()
            .collect();
    }
}

fn get_row(mut left: Tile, tiles: &[Tile], edge_map: &HashMap<Pat, Vec<usize>>) -> Vec<Tile> {
    let num_tiles = (tiles.len() as f32).sqrt() as usize;
    let mut row = vec![left.clone()];
    for _ in 1..num_tiles {
        let cands: Vec<Tile> = edge_map
            .get(&left.right())
            .unwrap()
            .iter()
            .filter(|id| **id != left.id)
            .map(|id| tiles.iter().find(|t| t.id == *id).unwrap())
            .cloned()
            .collect();
        assert!(cands.len() == 1);
        let cand = cands[0].clone().align_left(left.right());
        row.push(cand.clone());
        left = cand;
    }
    row
}

fn remove_borders(puzzle: Vec<Pat>, tile_size: usize) -> Vec<Pat> {
    let puzzle_size = puzzle.len();
    let mut padded = puzzle
        .into_iter()
        .map(|mut row| {
            row.insert(0, '.');
            row.push('.');
            row
        })
        .collect::<Vec<Pat>>();

    // Can be empty vecs but nice to see padding in print
    padded.insert(0, vec!['.'; puzzle_size + 2]);
    padded.push(vec!['.'; puzzle_size + 2]);
    padded
        .into_iter()
        .map(|row| {
            row.into_iter()
                .enumerate()
                .filter(|(idx, _col)| idx % tile_size > 1)
                .map(|(_idx, col)| col)
                .collect::<Pat>()
        })
        .enumerate()
        .filter(|(idx, _row)| idx % tile_size > 1)
        .map(|(_idx, row)| row)
        .collect::<Vec<Pat>>()
}

fn init_puzzle(tiles: &[Tile]) -> Vec<Pat> {
    let num_tiles = (tiles.len() as f32).sqrt() as usize;
    let tile_size = tiles[0].pat.len();
    let mut puzzle = Vec::new();
    for _ in 0..(num_tiles * tile_size) {
        puzzle.push(vec!['.'; num_tiles * tile_size]);
    }
    puzzle
}

// Push tile not tile.id
fn find_edge_tiles(tiles: &[Tile]) -> (Vec<usize>, Vec<usize>, HashMap<Pat, Vec<usize>>) {
    let mut edges = HashMap::new();
    for tile in tiles {
        for edge in tile.edge_perms() {
            let entry = edges.entry(edge).or_insert_with(Vec::new);
            entry.push(tile.id);
        }
    }

    let unique_count = tiles.iter().map(|t| {
        (
            t.id,
            t.edge_perms()
                .into_iter()
                .filter(|edge| edges.get(edge).unwrap().len() == 1)
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
        edges,
    )
}

#[derive(Debug, Clone)]
struct Monster {
    pat: Vec<Pat>,
}

impl Monster {
    fn rot_90(self) -> Monster {
        let (height, width) = self.size();
        let mut new_pat = Vec::new();
        for col in 0..width {
            let mut tmp = Vec::new();
            for row in (0..height).rev() {
                tmp.push(self.pat[row][col])
            }
            new_pat.push(tmp);
        }
        Monster { pat: new_pat }
    }

    fn flip(self) -> Monster {
        let new_pat = self.pat.into_iter().rev().collect();
        Monster { pat: new_pat }
    }

    fn size(&self) -> (usize, usize) {
        (self.pat.len(), self.pat[0].len())
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

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    pat: Vec<Pat>,
}

impl Tile {
    fn align_top(self, top: Pat) -> Tile {
        let mut tmp = self;
        for _ in 0..4 {
            tmp = tmp.rot_90();
            if tmp.top() == top {
                return tmp;
            }
        }
        tmp = tmp.flip();
        for _ in 0..4 {
            tmp = tmp.rot_90();
            if tmp.top() == top {
                return tmp;
            }
        }
        panic!("No match");
    }

    fn align_left(self, left: Pat) -> Tile {
        let mut tmp = self;
        for _ in 0..4 {
            tmp = tmp.rot_90();
            if tmp.left() == left {
                return tmp;
            }
        }
        tmp = tmp.flip();
        for _ in 0..4 {
            tmp = tmp.rot_90();
            if tmp.left() == left {
                return tmp;
            }
        }
        panic!("No match");
    }

    fn rot_90(self) -> Tile {
        let size = self.size();
        let mut new_pat = Vec::new();
        for col in 0..size {
            let mut tmp = Vec::new();
            for row in (0..size).rev() {
                tmp.push(self.pat[row][col])
            }
            new_pat.push(tmp);
        }
        Tile {
            id: self.id,
            pat: new_pat,
        }
    }

    fn flip(self) -> Tile {
        //let mut new_pat = Vec::new();
        //for row in 0..self.size() {
        //    new_pat.push(self.pat[row].clone().into_iter().rev().collect());
        //}
        let new_pat = self.pat.into_iter().rev().collect();
        Tile {
            id: self.id,
            pat: new_pat,
        }
    }

    fn from_input(id: usize, pat: Vec<Pat>) -> Tile {
        let size = pat.len();
        assert!(size == 10);
        Tile { id, pat }
    }

    fn left(&self) -> Pat {
        self.pat.iter().map(|row| row[0]).collect::<Pat>()
    }

    fn right(&self) -> Pat {
        self.pat
            .iter()
            .map(|row| row[self.size() - 1])
            .collect::<Pat>()
    }

    fn top(&self) -> Pat {
        self.pat[0].clone()
    }

    fn bot(&self) -> Pat {
        self.pat[self.size() - 1].clone()
    }

    fn size(&self) -> usize {
        self.pat.len()
    }

    fn edge_perms(&self) -> Vec<Pat> {
        let top = self.pat[0].clone();
        let bot = self.bot();
        let left = self.left();
        let right = self.right();
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

fn print_puzzle(puzzle: &[Pat]) {
    for row in puzzle {
        println!("{}", row.iter().collect::<String>());
    }
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
        tiles.push(Tile::from_input(id, pat))
    }
    tiles
}
