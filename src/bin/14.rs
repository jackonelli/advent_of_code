use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;

const ORE: &'static str = "ORE";
const FUEL: &'static str = "FUEL";

type Reactions = HashMap<String, (u64, HashMap<String, u64>)>;
fn parse_reactions(reactions_str: &str) -> Reactions {
    let reactions_re = Regex::new(r"((([0-9]+) ([A-Z]+))+)").expect("Invalid regex");
    let mut reactions = HashMap::new();
    for line in reactions_str.lines() {
        let mut tmp_vec = Vec::new();
        for cap in reactions_re.captures_iter(line) {
            let qty: u64 = cap[3].parse().expect("Incorrect qty");
            let id: String = cap[4].into();
            tmp_vec.push((id, qty));
        }
        let (result_id, result_qty) = tmp_vec.pop().expect("No result");

        let mut reactants: HashMap<String, u64> = HashMap::new();
        for (id, qty) in tmp_vec {
            reactants.insert(id.into(), qty);
        }
        reactions.insert(result_id.into(), (result_qty, reactants));
    }

    reactions
}

fn get_multiple_and_extra(min_qty: u64, needed_qty: u64) -> (u64, u64) {
    if needed_qty == 0 {
        return (0, 0);
    } else {
        let multiple = (needed_qty - 1) / min_qty + 1;
        let extra = multiple * min_qty - needed_qty;
        return (multiple, extra);
    }
}

fn reduce_totals(unreduced: VecDeque<(String, u64)>) -> VecDeque<(String, u64)> {
    let mut intermediate: HashMap<String, u64> = HashMap::new();
    for (id, qty) in unreduced {
        if intermediate.contains_key(&id) {
            let acc_qty = intermediate.get_mut(&id).expect("reduce get mut");
            *acc_qty += qty;
        } else {
            intermediate.insert(id.clone(), qty);
        }
    }
    let mut reduced = VecDeque::new();
    for (id, qty) in intermediate.into_iter() {
        reduced.push_back((id, qty))
    }
    reduced
}

fn calc_needed_and_extra(required_qty: u64, extra_qty: u64) -> (u64, u64) {
    if required_qty > extra_qty {
        (required_qty - extra_qty, 0)
    } else {
        (0, extra_qty - required_qty)
    }
}

fn get_total(
    totals: VecDeque<(String, u64)>,
    extra: HashMap<String, u64>,
    reactions: &Reactions,
) -> VecDeque<(String, u64)> {
    if totals.iter().all(|(id, qty)| id == ORE || *qty == 0) {
        totals
    } else {
        let mut new_totals = totals.clone();
        let mut new_extra = extra.clone();
        let (id, qty) = new_totals.pop_front().expect("Could not pop");
        if id == ORE {
            new_totals.push_back((id, qty));
        } else {
            let (min_qty, reactants) = &reactions[&id];
            let extra_qty = new_extra.remove(&id).unwrap_or(0);
            let (needed_qty, extra_qty) = calc_needed_and_extra(qty, extra_qty);
            let (multiple, add_extra) = get_multiple_and_extra(*min_qty, needed_qty);
            new_extra.insert(id.clone(), extra_qty + add_extra);
            new_totals = reactants.iter().fold(new_totals, |mut acc, (id, new_qty)| {
                acc.push_back((id.into(), *new_qty * multiple));
                acc
            });
            new_totals = reduce_totals(new_totals);
        }
        get_total(new_totals, new_extra, reactions)
    }
}

fn main() {
    env_logger::init();
    //star_1();
    star_2();
}

fn star_2() {
    let file = "input/14/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let reactions = parse_reactions(&contents);
    let ore_count: u64 = 1000_000_000_000;
    let mut upper = 5000000;
    let mut lower = 4000000;
    loop {
        let mut totals = VecDeque::new();
        let current = (upper + lower) / 2;
        totals.push_back((FUEL.into(), current));
        let extra = HashMap::new();
        let totals = get_total(totals.clone(), extra, &reactions);
        let ore_needed = totals.clone().pop_front().unwrap().1 as u64;
        if ore_needed > ore_count {
            upper = current;
        } else {
            lower = current;
        }
        println!("Ore needed: {}, Fuel: {}", ore_needed, current);
    }
}

fn star_1() {
    let file = "input/14/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let reactions = parse_reactions(&contents);
    let mut totals = VecDeque::new();
    totals.push_back((FUEL.into(), 1));
    let extra = HashMap::new();
    let mut totals = get_total(totals, extra, &reactions);
    println!("{}", totals.pop_front().unwrap().1);
}

#[cfg(test)]
mod tests_14 {
    use super::*;
    #[test]
    fn ex_1() {
        let file = "input/14/ex1";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let reactions = parse_reactions(&contents);
        let mut totals = VecDeque::new();
        totals.push_back((FUEL.into(), 1));
        let extra = HashMap::new();
        let mut totals = get_total(totals, extra, &reactions);
        assert_eq!(totals.pop_front().unwrap().1, 31);
    }
    #[test]
    fn ex_2() {
        let file = "input/14/ex2";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let reactions = parse_reactions(&contents);
        let mut totals = VecDeque::new();
        totals.push_back((FUEL.into(), 1));
        let extra = HashMap::new();
        let mut totals = get_total(totals, extra, &reactions);
        assert_eq!(totals.pop_front().unwrap().1, 165);
    }
    #[test]
    fn ex_3() {
        let file = "input/14/ex3";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let reactions = parse_reactions(&contents);
        let mut totals = VecDeque::new();
        totals.push_back((FUEL.into(), 1));
        let extra = HashMap::new();
        let mut totals = get_total(totals, extra, &reactions);
        assert_eq!(totals.pop_front().unwrap().1, 13312);
    }
}
