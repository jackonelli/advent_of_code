use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/5/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data = contents.trim().lines();

    let ids = data
        .map(|board| row_and_seat(board))
        .map(|(row, seat)| row * 8 + seat);

    let max = ids.clone().max().unwrap();
    println!("Star 1: {}", max);

    let ids: HashSet<u32> = ids.collect();
    // Unlikely that the set difference is a perf impr cmp to simply iterating the range.
    // Cooler, though.
    let all: HashSet<u32> = (2..max).collect();
    let my_id = all
        .difference(&ids)
        .find(|id| !ids.contains(id) && ids.contains(&(*id + 1)) && ids.contains(&(*id - 1)))
        .unwrap();
    println!("Star 2: {}", my_id);
}

fn row_and_seat(board: &str) -> (u32, u32) {
    let (row, seat) = board.split_at(7);
    (binary(row, 'B'), binary(seat, 'R'))
}

fn binary(bin: &str, cmp: char) -> u32 {
    bin.chars()
        .map(|c| (c == cmp) as u32)
        .fold(0, |x, b| x * 2 + b)
}
