use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/5/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let input = String::from(contents.trim());
    println!("{}", star_1(&input));
    println!("{}", star_2(&input));
}

fn star_1(input: &str) -> usize {
    react_string(input).len()
}

fn star_2(input: &str) -> usize {
    let shortest = input
        .chars()
        .unique()
        .map(|c| {
            (
                c,
                react_string(&strip_characters(&input, c.to_ascii_lowercase())).len(),
            )
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    shortest.1
}

fn react_string(input: &str) -> Vec<char> {
    input.chars().fold(Vec::new(), |mut acc, c| {
        if !acc.is_empty() && polar_opposite(&c, acc.last().unwrap()) {
            acc.pop();
        } else {
            acc.push(c);
        };
        acc
    })
}

fn strip_characters(original: &str, to_strip: char) -> String {
    original
        .chars()
        .filter(|&c| c != to_strip && c != to_strip.to_ascii_uppercase())
        .collect()
}

fn polar_opposite(a: &char, b: &char) -> bool {
    a != b && (a == &b.to_ascii_uppercase() || b == &a.to_ascii_uppercase())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn react_string_simple() {
        let input = String::from("dabAcCaCBAcCcaDA");
        let true_output = String::from("dabCBAcaDA");
        let output: String = react_string(&input).iter().collect();
        assert_eq!(output, true_output);
        assert_eq!(output.len(), 10);
    }
}
