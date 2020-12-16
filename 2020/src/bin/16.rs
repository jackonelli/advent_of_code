use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::RangeInclusive;

type Constraints = HashMap<String, Vec<RangeInclusive<usize>>>;

fn main() {
    let file = "input/16/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");

    let mut data = contents.trim().split("\n\n");
    let constr = data.next().unwrap();
    let constr = parse_constr(constr);
    let your = data.next().unwrap();
    let your = parse_your(your);
    let nearby = data.next().unwrap();
    let nearby = parse_nearby(nearby);
    println!("Star 1: {}", star_1(&constr, &nearby));
    println!("Star 2: {}", star_2(constr, &nearby, &your));
}

fn star_1(constraints: &Constraints, nearby: &[Vec<usize>]) -> usize {
    nearby
        .iter()
        .map(|ti| {
            ti.iter()
                .filter(|field| {
                    !constraints
                        .iter()
                        .map(|(_, constr)| constr)
                        .flatten()
                        .any(|constr| constr.contains(field))
                })
                .cloned()
                .collect::<Vec<usize>>()
        })
        .flatten()
        .sum()
}

fn star_2(constraints: Constraints, nearby: &[Vec<usize>], your: &[usize]) -> usize {
    let valid_tickets = nearby
        .iter()
        .filter(|ti| {
            ti.iter().all(|field| {
                constraints
                    .iter()
                    .map(|(_, constr)| constr)
                    .flatten()
                    .any(|constr| constr.contains(field))
            })
        })
        .cloned()
        .collect::<Vec<Vec<usize>>>();

    let mut constraints = constraints.clone();
    let mut your_keys = HashMap::new();
    while !constraints.is_empty() {
        for idx in 0..your.len() {
            let key = find_constr(&constraints, &valid_tickets, idx);
            if key.len() == 1 {
                constraints.remove(&key[0]);
                your_keys.insert(key[0].clone(), your[idx]);
            }
        }
    }
    your_keys
        .iter()
        .filter(|(key, _val)| key.contains("departure"))
        .map(|(_key, val)| *val)
        .product()
}

fn find_constr(constraints: &Constraints, valid_tickets: &[Vec<usize>], idx: usize) -> Vec<String> {
    let candidates: Vec<String> = constraints
        .iter()
        .filter(|(_, constr)| {
            valid_tickets
                .iter()
                .map(|ti| ti[idx])
                .all(|field| constr.iter().any(|c| c.contains(&field)))
        })
        .map(|(key, c)| key.clone())
        .collect();
    candidates
}

fn parse_constr(constr: &str) -> Constraints {
    let re = Regex::new(r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let mut parsed_constr = HashMap::new();
    for line in constr.trim().lines() {
        let cap = re.captures(line).unwrap();
        let key = String::from(cap.get(1).unwrap().as_str());
        let l1 = cap_to_int(&cap, 2);
        let h1 = cap_to_int(&cap, 3);
        let l2 = cap_to_int(&cap, 4);
        let h2 = cap_to_int(&cap, 5);
        parsed_constr.insert(key, vec![(l1..=h1), (l2..=h2)]);
    }
    parsed_constr
}

fn parse_nearby(nearby: &str) -> Vec<Vec<usize>> {
    let mut lines = nearby.trim().lines();
    lines.next();
    let mut res: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        res.push(
            line.split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect(),
        );
    }
    res
}

fn parse_your(your: &str) -> Vec<usize> {
    let mut lines = your.trim().lines();
    lines.next();
    lines
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn cap_to_int(cap: &Captures, num: usize) -> usize {
    cap.get(num).unwrap().as_str().parse::<usize>().unwrap()
}
