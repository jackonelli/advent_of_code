use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let (window, file) = (25, "input/9/input");
    // let (window, file) = (5, "input/9/test");
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
    println!("Star 2: {}", star_2(&data, inv));
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

fn star_2(numbers: &[usize], target: usize) -> usize {
    let seq = caterpillar(numbers, target);
    let largest = seq.iter().max().expect("No max");
    let smallest = seq.iter().min().expect("No max");
    smallest + largest
}

fn caterpillar(numbers: &[usize], target: usize) -> &[usize] {
    let mut tail = 0;
    let mut head = 2;
    let mut sum: usize = numbers[tail..head].iter().sum();
    loop {
        assert!(head < numbers.len());
        while tail < head && sum + numbers[head] > target {
            sum -= numbers[tail];
            tail += 1;
        }
        while sum + numbers[head] <= target {
            sum += numbers[head];
            head += 1;
        }
        if sum == target {
            break;
        }
    }

    &numbers[tail..head]
}

fn _brute(numbers: &[usize], sum: usize) -> usize {
    // Starting from 3 since, in effect, consecutive sum of two is included in the first star.
    let (small, large) = (3..(numbers.len() + 1))
        .map(|window| _find_consec_sum(numbers, window, sum))
        .find(|res| res.is_some())
        .expect("Unwrap to see if 'find' found something")
        .expect("Found no consec sum.");
    small + large
}

fn _find_consec_sum(numbers: &[usize], window: usize, sum: usize) -> Option<(usize, usize)> {
    match (0..(numbers.len() - window - 1))
        .map(|i| (i, numbers[i..window + i].iter().sum::<usize>()))
        .find(|(_, consec_sum)| *consec_sum == sum)
    {
        Some((i, _)) => {
            let largest = numbers[i..window + i].iter().max().expect("No max");
            let smallest = numbers[i..window + i].iter().min().expect("No max");
            Some((*smallest, *largest))
        }
        None => None,
    }
}
