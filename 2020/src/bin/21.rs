use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/21/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");

    let list = parse_lines(&contents);
    let dict = gen_allergen_dict(&list);
    println!("Star 1:  {}", star_1(&list, &dict));
    println!("Star 2: '{}'", star_2(&dict));
}

fn parse_lines(contents: &str) -> Vec<(HashSet<String>, Vec<String>)> {
    let lines = contents.trim().lines();
    let mut list = Vec::new();
    for l in lines {
        let mut parts = l.split(" (contains ");
        let ingreds = parts.next().unwrap().split(' ').map(String::from).collect();
        let mut allergs = String::from(parts.next().unwrap());
        // Drop last ')'
        allergs.pop();
        let allergs = allergs.split(", ").map(String::from).collect();
        list.push((ingreds, allergs));
    }
    list
}
fn gen_allergen_dict(list: &[(HashSet<String>, Vec<String>)]) -> HashMap<String, String> {
    let dict: HashMap<String, HashSet<String>> =
        list.iter().fold(HashMap::new(), |acc, (ingreds, allergs)| {
            allergs.iter().fold(acc, |mut sub_acc, all| {
                match sub_acc.get_mut(all) {
                    Some(set) => {
                        let tmp: HashSet<String> = set.intersection(ingreds).cloned().collect();
                        *set = tmp;
                    }
                    None => {
                        sub_acc.insert(all.clone(), ingreds.clone());
                    }
                }
                sub_acc
            })
        });

    let mut sort_dict: Vec<(String, HashSet<String>)> = dict
        .into_iter()
        .map(|(all, ingreds)| (all, ingreds))
        .collect();

    let mut allergen_dict = HashMap::new();
    // The dict now has pairs:  (all: {ingreds}).
    // Hoping that the input is s.t. that there exists at least one entry s.t. |{ingreds}| = 1.
    // Sorting on set size will bring this entry to the front.
    // Remove it to get the first unique match. Remove all instances of this match in all entries.
    // Hope that the resulting dict again has a least one entry with a singleton set.
    // Repeat.
    for _ in 0..sort_dict.len() {
        sort_dict.sort_by_key(|(_all, ingreds)| ingreds.len());
        let (all, candidates) = sort_dict.remove(0);
        assert!(candidates.len() == 1);
        let match_ = candidates.iter().next().unwrap().clone();
        for (_all, candidates) in sort_dict.iter_mut() {
            candidates.remove(&match_);
        }
        allergen_dict.insert(all, match_);
    }
    allergen_dict
}

fn star_1(
    list: &[(HashSet<String>, Vec<String>)],
    allergen_dict: &HashMap<String, String>,
) -> usize {
    let dict = allergen_dict
        .iter()
        .map(|(all, ingred)| (ingred.clone(), all.clone()))
        .collect::<HashMap<String, String>>();
    list.iter()
        .map(|(ingreds, _all)| {
            ingreds
                .iter()
                .filter(|ingred| !dict.contains_key(*ingred))
                .count()
        })
        .sum()
}

fn star_2(allergen_dict: &HashMap<String, String>) -> String {
    let mut dict: Vec<(String, String)> = allergen_dict.clone().into_iter().collect();
    dict.sort_by_key(|(all, _ingred)| all.clone());

    dict.into_iter()
        .map(|(_all, ingred)| ingred)
        .collect::<Vec<String>>()
        .join(",")
}
