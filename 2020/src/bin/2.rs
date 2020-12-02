#![feature(test)]
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/2/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data = contents.trim().lines();

    println!("Star 1: {}", count_valid_passwds(data.clone(), char_count));
    println!("Star 2: {}", count_valid_passwds(data, char_pos));
}

fn count_valid_passwds(
    data: std::str::Lines,
    predicate: fn(&char, &usize, &usize, &str) -> bool,
) -> usize {
    data.map(split_input)
        .filter(|(l, min, max, pass)| predicate(l, min, max, pass))
        .count()
}

fn char_pos(l: &char, pos1: &usize, pos2: &usize, pass: &str) -> bool {
    let pass: Vec<char> = pass.chars().collect();
    //One indexed positions.
    let c1 = pass[*pos1 - 1] == *l;
    let c2 = pass[*pos2 - 1] == *l;
    c1 ^ c2
}

fn char_count(l: &char, min: &usize, max: &usize, pass: &str) -> bool {
    let count = pass.matches(*l).count();
    count >= *min && count <= *max
}

/// Need to learn how to use regex
fn _re_split(input: &str) -> (char, usize, usize, String) {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let cap = re.captures(input).unwrap();
    let d1 = cap[1].parse().expect("d1");
    let d2 = cap[2].parse().expect("d2");
    let l = cap[3].chars().next().expect("l");
    let pass = String::from(&cap[4]);
    (l, d1, d2, pass)
}

fn split_input(input: &str) -> (char, usize, usize, &str) {
    let mut tmp = input.split(": ");
    let (rule, pass) = (tmp.next().expect("rule"), tmp.next().expect("pass"));
    let mut tmp = rule.split(' ');
    let (counts, letter) = (tmp.next().expect("counts"), tmp.next().expect("letter"));
    let mut tmp = counts.split('-');
    let (min, max) = (
        tmp.next()
            .expect("min")
            .parse::<usize>()
            .expect("int parse"),
        tmp.next()
            .expect("max")
            .parse::<usize>()
            .expect("int parse"),
    );
    (
        letter.chars().next().expect("letter to char"),
        min,
        max,
        pass,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    extern crate test;
    #[bench]
    fn day_2_star_1(b: &mut Bencher) {
        let file = "input/2/input";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let data = contents.trim().lines();
        b.iter(|| count_valid_passwds(data.clone(), char_count))
    }

    #[bench]
    fn day_2_star_2(b: &mut Bencher) {
        let file = "input/2/input";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let data = contents.trim().lines();
        b.iter(|| {
            let count = count_valid_passwds(data.clone(), char_pos);
            assert_eq!(count, 489)
        })
    }
    #[bench]
    fn read_data(b: &mut Bencher) {
        let file = test::black_box("input/2/input");
        b.iter(|| {
            let mut file = File::open(file).expect("Opening file error");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Read to string error");
            let _data = contents.trim().lines();
        })
    }
}
