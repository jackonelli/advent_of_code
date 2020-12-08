#![feature(test)]
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/8/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let input = contents.trim().lines();
    let instrs = parse_instrs(input);
    println!("Star 1: {:?}", run(&instrs, 0, 0, &mut HashSet::new()));
    println!("Star 2: {}", modified_code_run(instrs))
}

fn modified_code_run(instrs: Vec<Instr>) -> i32 {
    let mut instrs = instrs;
    for idx in 0..instrs.len() - 1 {
        let old_instr = instrs[idx].clone();
        instrs[idx] = instrs[idx].jmp_nop_swap();
        let res = run(&instrs, 0, 0, &mut HashSet::new());
        match res {
            Res::Term(val) => return val,
            Res::Inf(_) => {}
        }
        instrs[idx] = old_instr;
    }
    panic!("reached end");
}

fn run(instrs: &[Instr], idx: usize, acc: i32, visited: &mut HashSet<usize>) -> Res {
    if idx == instrs.len() {
        return Res::Term(acc);
    }

    if visited.contains(&idx) {
        return Res::Inf(acc);
    }
    visited.insert(idx);

    match instrs[idx] {
        Instr::Nop(_val) => run(instrs, idx + 1, acc, visited),
        Instr::Acc(val) => run(instrs, idx + 1, acc + val, visited),
        Instr::Jmp(val) => run(instrs, (idx as i32 + val) as usize, acc, visited),
    }
}

fn parse_instrs(lines: std::str::Lines) -> Vec<Instr> {
    let reg = Regex::new(r"^(\w+) ([+,-]\d+)").expect("Invalid regex");
    lines
        .map(|line| {
            let matches = reg.captures(line).expect("no match");
            Instr::from_match(matches)
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
pub enum Res {
    Inf(i32),
    Term(i32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instr {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instr {
    fn from_match(matches: regex::Captures) -> Instr {
        let val = matches
            .get(2)
            .expect("no color")
            .as_str()
            .parse::<i32>()
            .unwrap();
        match matches.get(1).expect("no instr").as_str() {
            "nop" => Instr::Nop(val),
            "acc" => Instr::Acc(val),
            "jmp" => Instr::Jmp(val),
            _ => panic!("Unknown instr"),
        }
    }

    fn jmp_nop_swap(&self) -> Instr {
        match self {
            Instr::Nop(val) => Instr::Jmp(*val),
            Instr::Acc(val) => Instr::Acc(*val),
            Instr::Jmp(val) => Instr::Nop(*val),
        }
    }
}
