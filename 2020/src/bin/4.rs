use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/4/input";
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
    let acc = acc.iter().map(|rec| String::from(rec.trim()));
    println!(
        "Star 1: {}",
        acc.clone().filter(|rec| rec_valid(rec)).count()
    );
    println!("Star 2: {}", star_2(acc));
}

fn rec_valid(rec: &str) -> bool {
    let ids = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    ids.iter().all(|id| rec.contains(id))
}

fn star_2(acc: impl Iterator<Item = String>) -> usize {
    acc.filter(|rec| rec_valid(rec))
        .filter(|rec| check_rec(rec))
        .count()
}

fn check_rec(rec: &str) -> bool {
    rec.split(' ').all(|field| rules(field))
}

fn rules(field: &str) -> bool {
    let mut aba = field.split(':');
    let (key, val) = (
        aba.next().expect("key"),
        aba.next().unwrap_or_else(|| panic!("val '{}'", field)),
    );
    match key {
        "cid" => true,
        "byr" => {
            let byr = val.parse::<usize>().expect("byr parse");
            (1920..=2002).contains(&byr)
        }
        "iyr" => {
            let iyr = val
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("iyr parse {}", val));
            (2010..=2020).contains(&iyr)
        }
        "eyr" => {
            let eyr = val.parse::<usize>().expect("eyr parse");
            (2020..=2030).contains(&eyr)
        }
        "hgt" => {
            if val.contains("in") {
                let mut h = val.split('i');
                let hgt = h
                    .next()
                    .expect("hgt parse")
                    .parse::<usize>()
                    .expect("hgt usize");
                (59..=76).contains(&hgt)
            } else {
                let mut h = val.split('c');
                let hgt = h
                    .next()
                    .expect("hgt parse")
                    .parse::<usize>()
                    .expect("hgt usize");
                (150..=193).contains(&hgt)
            }
        }
        "hcl" => {
            let len = val.len();
            let mut cs = val.chars();
            cs.next().expect("no first") == '#' && cs.all(|c| c.is_alphanumeric()) && len == 7
        }
        "ecl" => {
            let ids = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            ids.iter().any(|id| val.contains(id))
        }
        "pid" => val.len() == 9 && val.chars().all(|c| c.is_numeric()),
        _ => panic!("no match {}", field),
    }
}
