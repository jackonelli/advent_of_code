use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn parse_reactions(reactions_str: &str) -> HashMap<String, HashMap<String, u32>> {
    let reactions_re = Regex::new(r"^((([0-9]+) ([A-Z]+)[, ]{0,2})+) => ([0-9]+) ([A-Z]+)$")
        .expect("Invalid regex");
    let reactions = HashMap::new();
    for line in reactions_str.lines() {
        //println!("{}", line.trim());
        for cap in reactions_re.captures_iter(line) {
            let reactant: String = cap[5].into();
            println!("{}", reactant)
        }
    }

    reactions
}

fn main() {
    env_logger::init();

    let file = "input/14/ex3";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    parse_reactions(&contents);
}

#[cfg(test)]
mod tests_14 {
    use super::*;
    #[test]
    fn aba() {}
}
