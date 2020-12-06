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
    let data = contents.trim().lines();
    let (acc, last_rec): (Vec<String>, String) =
        data.fold((Vec::new(), String::new()), |(mut list, mut rec), l| {
            if l.is_empty() {
                list.push(rec);
                (list, String::new())
            } else {
                rec.push_str(l);
                rec.push(' ');
                (list, rec)
            }
        });
    let acc = [&acc[..], &[String::from(last_rec.trim())]].concat();
    let count = count_groups(&acc);
    println!("{}", count);
}

fn count_groups(groups: &Vec<String>) -> usize {
    groups.iter().map(|g| count_group(g)).sum()
}

fn count_group(group: &str) -> usize {
    let mut sets = group
        .split(' ')
        .map(|p| count_person(p))
        .filter(|p| !p.is_empty());
    let first = sets.next().unwrap();
    let count = sets.fold(first, |acc, pers| {
        acc.intersection(&pers)
            .map(|x| *x)
            .collect::<HashSet<char>>()
    });
    count.len()
}

fn count_person(person: &str) -> HashSet<char> {
    person.chars().collect()
}

fn star_2(acc: impl Iterator<Item = String>) -> usize {
    1
}
