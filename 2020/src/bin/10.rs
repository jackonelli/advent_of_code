#![feature(test)]
#![feature(split_inclusive)]
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/10/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data: Vec<usize> = contents
        .trim()
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let mut nums = data;
    nums.sort_unstable();
    println!("Star 1: {}", star_1(&nums));
    nums.insert(0, 0);
    nums.push(*nums.iter().max().unwrap());
    println!("Star 2: {}", star_2(&nums));
}

fn star_1(nums: &[usize]) -> usize {
    let mut prev = 0;
    let mut ones = 0;
    let mut threes = 0;
    for d in nums {
        match d - prev {
            1 => ones += 1,
            3 => threes += 1,
            _ => panic!("wrong diff"),
        }
        prev = *d;
    }
    ones * (threes + 1)
}

fn star_2(nums: &[usize]) -> usize {
    let diffs = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(first, second)| second - first)
        .collect::<Vec<usize>>();
    diffs
        .split_inclusive(|diff| *diff == 3)
        .map(|sub_graph| sub_graph.len())
        .map(paths_in_graph)
        .product()
}

fn paths_in_graph(order: usize) -> usize {
    match order {
        5 => 7,
        4 => 4,
        3 => 2,
        0..=2 => 1,
        _ => panic!("did not think this far."),
    }
}
