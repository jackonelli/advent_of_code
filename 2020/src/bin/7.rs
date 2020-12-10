#![feature(test)]
use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;
use petgraph::Direction;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

type Idx = NodeIndex;
type Graph = StableDiGraph<String, usize>;
type RevIMap = HashMap<String, Idx>;

fn main() {
    let file = "input/7/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let input = contents;
    let (graph, rev_i_map) = parse_graph(input.lines());
    let count = parents(&graph, rev_i_map.get("shiny gold").unwrap()).len();
    println!("Star 1: {}", count);
    let bag_count = count_bags(&graph, rev_i_map.get("shiny gold").unwrap());
    // Rec function count includes root, answer need to be reduced by 1
    println!("Star 2: {}", bag_count - 1)
}

fn count_bags(graph: &Graph, idx: &Idx) -> usize {
    let children: HashSet<Idx> = graph
        .neighbors_directed(*idx, Direction::Outgoing)
        .collect();
    if children.len() == 0 {
        1
    } else {
        children
            .iter()
            .map(|child| {
                let edge = graph.find_edge(*idx, *child).unwrap();
                let count = graph[edge];
                count * count_bags(graph, &child)
            })
            .sum::<usize>()
            + 1
    }
}

fn parents(graph: &Graph, idx: &Idx) -> HashSet<Idx> {
    let new_p: HashSet<Idx> = graph
        .neighbors_directed(*idx, Direction::Incoming)
        .collect();
    if new_p.len() == 0 {
        HashSet::new()
    } else {
        new_p.clone().iter().fold(new_p, |mut acc_parents, idx| {
            acc_parents = acc_parents.union(&parents(graph, &idx)).cloned().collect();
            acc_parents
        })
    }
}

fn parse_line(line: &str) -> (String, Vec<(usize, String)>) {
    let mut sp = line.split(" bags contain ");
    let parent = sp.next().expect("parse parent");
    let children = sp.next().expect("parse children");
    if children.contains("no other bags") {
        (String::from(parent), Vec::new())
    } else {
        let children = children.split(',');
        let reg = Regex::new(r"(\d+) (\w+ \w+)").expect("Invalid regex");
        let children: Vec<(usize, String)> = children
            .map(|s| {
                let matches = reg.captures(s).expect("no match");
                (
                    matches
                        .get(1)
                        .expect("no num")
                        .as_str()
                        .parse::<usize>()
                        .expect("num parse"),
                    String::from(matches.get(2).expect("no color").as_str()),
                )
            })
            .collect();
        (String::from(parent), children)
    }
}

fn parse_graph(bag_rules: std::str::Lines) -> (Graph, RevIMap) {
    let bag_rules = bag_rules.map(parse_line);
    let mut graph = Graph::new();
    let mut rev_i_map = RevIMap::new();
    for (parent, children) in bag_rules {
        let p_idx = if !rev_i_map.contains_key(&parent) {
            let p_idx = graph.add_node(parent.clone());
            rev_i_map.insert(parent, p_idx);
            p_idx
        } else {
            *rev_i_map.get(&parent).unwrap()
        };
        for (count, child) in children {
            let c_idx = if !rev_i_map.contains_key(&child) {
                let c_idx = graph.add_node(child.clone());
                rev_i_map.insert(child, c_idx);
                c_idx
            } else {
                *rev_i_map.get(&child).unwrap()
            };
            graph.add_edge(p_idx, c_idx, count);
        }
    }
    return (graph, rev_i_map);
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use test::Bencher;
//    extern crate test;
//    #[bench]
//    fn star_1(b: &mut Bencher) {
//        let file = "input/7/input";
//        let mut file = File::open(file).expect("Opening file error");
//        let mut contents = String::new();
//        file.read_to_string(&mut contents)
//            .expect("Read to string error");
//        let input = contents;
//        let (graph, rev_i_map) = parse_graph(input.lines());
//        b.iter(|| parents(&graph, rev_i_map.get("shiny gold").unwrap()).len())
//    }
//    #[bench]
//    fn star_2(b: &mut Bencher) {
//        let file = "input/7/input";
//        let mut file = File::open(file).expect("Opening file error");
//        let mut contents = String::new();
//        file.read_to_string(&mut contents)
//            .expect("Read to string error");
//        let input = contents;
//        let (graph, rev_i_map) = parse_graph(input.lines());
//        b.iter(|| count_bags(&graph, rev_i_map.get("shiny gold").unwrap()))
//    }
//
//    #[bench]
//    fn read_data(b: &mut Bencher) {
//        let file = "input/7/input";
//        let mut file = File::open(file).expect("Opening file error");
//        let mut contents = String::new();
//        file.read_to_string(&mut contents)
//            .expect("Read to string error");
//        let input = contents;
//        b.iter(|| parse_graph(input.lines()))
//    }
//}
