use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/6/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");

    let data = contents.trim().split("\n\n");
    let groups: Vec<Vec<HashSet<char>>> = data
        .map(|group| {
            group
                .split('\n')
                .map(|person| person.chars().collect::<HashSet<char>>())
                .collect::<Vec<HashSet<char>>>()
        })
        .collect();
    println!("Star 1: {}", star_1(&groups));
    println!("Star 2: {}", star_2(&groups));
}

fn star_1(group: &[Vec<HashSet<char>>]) -> usize {
    group.iter().map(|g| unique_ans_in_group(g)).sum()
}

fn star_2(group: &[Vec<HashSet<char>>]) -> usize {
    group.iter().map(|g| common_ans_in_group(g)).sum()
}

fn unique_ans_in_group(group: &[HashSet<char>]) -> usize {
    group.iter().flatten().unique().count()
}

fn common_ans_in_group(group: &[HashSet<char>]) -> usize {
    let mut sets = group.iter();
    let first = sets.next().unwrap().clone();
    sets.fold(first, |acc, pers| {
        acc.intersection(&pers).cloned().collect::<HashSet<char>>()
    })
    .len()
}
