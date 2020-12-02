use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/1/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data: Vec<usize> = contents
        .trim()
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let (f1, f2) = data
        .iter()
        .combinations_with_replacement(2)
        .map(|comb| (*comb[0], *comb[1]))
        .find(|(f1, f2)| f1 + f2 == 2020)
        .expect("No prod == 2020");
    println!("Star 1: {}", f1 * f2);

    let (f1, f2, f3) = data
        .iter()
        .combinations_with_replacement(3)
        .map(|comb| (*comb[0], *comb[1], *comb[2]))
        .find(|(f1, f2, f3)| f1 + f2 + f3 == 2020)
        .expect("No prod == 2020");
    println!("Star 2: {}", f1 * f2 * f3);
}
