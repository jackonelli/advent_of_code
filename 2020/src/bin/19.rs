use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::Split;

type Rules = HashMap<usize, Rule>;
type Candidates = Vec<String>;

fn main() {
    let file = "input/19/input";
    //let file = "input/19/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");

    let data = contents.trim().split("\n\n");
    println!("Star 1: {}", star_1(data.clone()));
    println!("Star 2: {}", star_2(data.clone()));
}

fn cyclic(rules: &Rules) -> Vec<Regex> {
    let re_42 = find_re(42, rules, String::new());
    let re_31 = find_re(31, rules, String::new());
    let mut res = Vec::new();
    // I want a regex on this form:
    // "^((42)+)((42)+(31)+)$"
    // but this allows for false positives where rule 11 has different number of reps of 42 compared to the number of reps of 31.
    // Instead: do this bastard count where I make a set (of ad hoc length) with exact regexes.
    for i in 1..5 {
        let cand = format!(
            r"^(({})+)(({}){{{}}}({}){{{}}})$",
            re_42, re_42, i, re_31, i
        );
        res.push(Regex::new(&cand).unwrap());
    }
    res
}

fn star_1(mut data: Split<&str>) -> usize {
    let rules = parse_rules(data.next().unwrap());
    let re = find_re(0, &rules, String::new());
    let re = Regex::new(&format!("^{}$", re)).unwrap();
    parse_cands(data.next().unwrap())
        .into_iter()
        .filter(|cand| re.is_match(cand))
        .count()
}

fn star_2(mut data: Split<&str>) -> usize {
    let rules = parse_rules(data.next().unwrap());
    let res = cyclic(&rules);
    parse_cands(data.next().unwrap())
        .into_iter()
        .filter(|cand| res.iter().any(|re| re.is_match(cand)))
        .count()
}

fn find_re(rule_num: usize, rules: &Rules, mut re: String) -> String {
    match rules.get(&rule_num).expect("Rule num").clone() {
        Rule::Char(c) => {
            re.push_str(&c.to_string());
            re
        }

        Rule::List(list) => {
            for rule_num in list {
                re = find_re(rule_num, rules, re.clone());
            }
            re
        }
        Rule::OrLi((first, second)) => {
            let mut tmp = String::new();
            for rule_num in first {
                tmp = find_re(rule_num, rules, tmp);
            }
            let first_split = tmp;

            let mut tmp = String::new();
            for rule_num in second {
                tmp = find_re(rule_num, rules, tmp);
            }
            let new_re = format!("{}({}|{})", re, first_split, tmp);
            new_re
        }
    }
}

fn parse_rules(rules: &str) -> Rules {
    let rules = rules.trim().lines();
    let mut parsed_rules = Rules::new();

    for line in rules {
        let mut tmp = line.split(": ");
        let rule_num = tmp
            .next()
            .expect("No rule num")
            .parse::<usize>()
            .expect("rule num parse");
        let rule = Rule::from(tmp.next().expect("after colon"));
        parsed_rules.insert(rule_num, rule);
    }
    parsed_rules
}

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    List(Vec<usize>),
    OrLi((Vec<usize>, Vec<usize>)),
}

impl Rule {
    fn from(rule: &str) -> Self {
        if rule.contains('|') {
            let mut tmp = rule.split(" | ");
            let first: Vec<usize> = tmp
                .next()
                .expect("first_part")
                .split(' ')
                .map(|d| d.parse::<usize>().expect("dig parse"))
                .collect();
            let second: Vec<usize> = tmp
                .next()
                .expect("second")
                .split(' ')
                .map(|d| d.parse::<usize>().expect("dig parse"))
                .collect();
            Rule::OrLi((first, second))
        } else if rule.contains('a') {
            Rule::Char('a')
        } else if rule.contains('b') {
            Rule::Char('b')
        } else {
            Rule::List(
                rule.split(' ')
                    .map(|d| d.parse::<usize>().expect("list dig"))
                    .collect(),
            )
        }
    }
}

fn parse_cands(cands: &str) -> Candidates {
    cands.trim().lines().map(String::from).collect()
}
