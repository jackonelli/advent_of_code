#![feature(test)]
use std::collections::HashMap;
fn main() {
    let seq = &[14, 3, 1, 0, 9, 5];

    println!("Star 1: {}", get_nth(seq, 2020));
    println!("Star 2: {}", get_nth(seq, 30_000_000));
}

fn get_nth(seq: &[usize], nth: usize) -> usize {
    let seed = seq
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (t, d)| {
            acc.insert(*d, t + 1);
            acc
        });
    let (_, last_spoken) = (seq.len() + 1..nth).fold((seed, 0), |(mut prev, spoken), t| {
        let new_spoken = match prev.get(&spoken) {
            Some(last) => (t - last),
            None => 0,
        };
        prev.insert(spoken, t);
        (prev, new_spoken)
    });
    last_spoken
}
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    extern crate test;
    #[bench]
    fn day_15_star_1(b: &mut Bencher) {
        let seq = &[14, 3, 1, 0, 9, 5];
        b.iter(|| get_nth(seq, 2020))
    }
}
