use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::RangeInclusive;

type Constraints = HashMap<String, Vec<RangeInclusive<usize>>>;
type Ticket = Vec<usize>;
type TicketRef<'a> = &'a [usize];

fn main() {
    let file = "input/16/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");

    let mut data = contents.trim().split("\n\n");
    let constr = parse_constr(data.next().unwrap());
    let your = parse_your(data.next().unwrap());
    let nearby = parse_nearby(data.next().unwrap());
    println!("Star 1: {}", star_1(&constr, &nearby));
    println!("Star 2: {}", star_2(&constr, nearby, &your));
}

fn star_1(constraints: &Constraints, nearby: &[Ticket]) -> usize {
    nearby
        .iter()
        .map(|ticket| {
            ticket.iter().filter(|field| {
                !constraints
                    .iter()
                    .map(|(_, constr)| constr)
                    .flatten()
                    .any(|constr| constr.contains(field))
            })
        })
        .flatten()
        .sum()
}

fn star_2(constraints: &Constraints, nearby: Vec<Ticket>, your: TicketRef) -> usize {
    let valid_tickets = nearby
        .into_iter()
        .filter(|ti| {
            ti.iter().all(|field| {
                constraints
                    .iter()
                    .map(|(_, constr)| constr)
                    .flatten()
                    .any(|constr| constr.contains(field))
            })
        })
        .collect::<Vec<Vec<usize>>>();

    let mut idx_candidates: Vec<(usize, Vec<String>)> = (0..your.len())
        .map(|idx| (idx, find_constr(&constraints, &valid_tickets, idx)))
        .collect();
    idx_candidates.sort_unstable_by_key(|(_idx, keys)| keys.len());

    let key_idx_map =
        idx_candidates
            .iter()
            .fold(HashMap::new(), |mut solved_keys, (idx, candidates)| {
                let mut rem_keys = candidates.iter().filter(|k| !solved_keys.contains_key(*k));
                let key = rem_keys
                    .next()
                    .unwrap_or_else(|| panic!("No candidate for idx: {}", idx));
                if rem_keys.count() == 0 {
                    solved_keys.insert(key, *idx);
                } else {
                    panic!("Ambiguous key.")
                }
                solved_keys
            });

    key_idx_map
        .iter()
        .filter(|(key, _idx)| key.contains("departure"))
        .map(|(_key, idx)| your[*idx])
        .product()
}

fn find_constr(constraints: &Constraints, valid_tickets: &[Ticket], idx: usize) -> Vec<String> {
    let candidates: Vec<String> = constraints
        .iter()
        .filter(|(_, constr)| {
            valid_tickets
                .iter()
                .map(|ti| ti[idx])
                .all(|field| constr.iter().any(|c| c.contains(&field)))
        })
        .map(|(key, _c)| key.clone())
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

fn parse_nearby(nearby: &str) -> Vec<Ticket> {
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

fn parse_your(your: &str) -> Ticket {
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
