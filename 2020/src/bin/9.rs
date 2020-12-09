use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let (window, file) = (25, "input/9/input");
    //let (window, file) = (5, "input/9/test");
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data: Vec<usize> = contents
        .trim()
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let inv = first_invalid(&data, window);
    println!("Star 1: {}", inv);
    println!("Star 2: {}", brute(&data, inv));
}

fn first_invalid(numbers: &[usize], window: usize) -> usize {
    let inv_pos = (0..(numbers.len() - 1))
        .find(|i| !sum_exists(&numbers[*i..window + i], numbers[window + i]))
        .expect("No invalid");
    numbers[window + inv_pos]
}

fn sum_exists(numbers: &[usize], sum: usize) -> bool {
    numbers
        .iter()
        .combinations_with_replacement(2)
        .any(|pair| pair[0] + pair[1] == sum)
}

fn brute(numbers: &[usize], sum: usize) -> usize {
    // Starting from 3 since, in effect, consecutive sum of two is included in the first star.
    let (small, large) = (3..(numbers.len() + 1))
        .map(|window| find_consec_sum(numbers, window, sum))
        .find(|res| res.is_some())
        .expect("Unwrap to see if 'find' found something")
        .expect("Found no consec sum.");
    small + large
}

fn find_consec_sum(numbers: &[usize], window: usize, sum: usize) -> Option<(usize, usize)> {
    match (0..(numbers.len() - window - 1))
        .map(|i| (i, numbers[i..window + i].iter().sum::<usize>()))
        .find(|(_, consec_sum)| *consec_sum == sum)
    {
        Some((i, _)) => {
            let mut vec = numbers[i..window + i].to_vec();
            vec.sort_unstable();
            Some((
                *vec.first().expect("No first"),
                *vec.last().expect("No last"),
            ))
        }
        None => None,
    }
}
