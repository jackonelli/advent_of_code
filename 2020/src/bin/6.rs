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
    let groups = data.map(|group| {
        group
            .split('\n')
            .map(|person| person.chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>()
    });
    println!("Star 1: {}", star_1(groups.clone()));
    println!("Star 2: {}", star_2(groups));
}

fn star_1(group: impl Iterator<Item = Vec<HashSet<char>>>) -> usize {
    group.map(|g| unique_ans_in_group(&g)).sum()
}

fn star_2(group: impl Iterator<Item = Vec<HashSet<char>>>) -> usize {
    group.map(|g| common_ans_in_group(&g)).sum()
}

fn unique_ans_in_group(group: &[HashSet<char>]) -> usize {
    group.iter().flatten().unique().count()
}

// Did my own impl of repeated intersection.
// All lot of allocation so prolly slower but good practice.
fn common_ans_in_group(group: &[HashSet<char>]) -> usize {
    //let mut sets = group.iter();
    //let first = sets.next().unwrap().clone();
    //sets.fold(first, |acc, pers| {
    //    acc.intersection(&pers).cloned().collect::<HashSet<char>>()
    //})
    let count = rep_intersects(group.iter().cloned()).len();
    count
}

fn rep_intersects(sets: impl Iterator<Item = HashSet<char>> + Clone) -> HashSet<char> {
    let size = sets.clone().count();
    match size {
        0 => HashSet::new(),
        1 => {
            let mut sets = sets;
            sets.next().unwrap()
        }
        _ => {
            let mut sets = sets;
            let first = sets.next().unwrap();
            let intersection = sets.scan(first, |intersection, set| {
                let new_inter = intersection
                    .intersection(&set)
                    .cloned()
                    .collect::<HashSet<char>>();
                *intersection = new_inter.clone();
                Some(new_inter)
            });
            match intersection.clone().find(|set| set.is_empty()) {
                Some(_) => HashSet::new(),
                None => intersection.last().unwrap(),
            }
        }
    }
}
