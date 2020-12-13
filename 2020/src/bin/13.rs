#![feature(test)]
#![feature(split_inclusive)]
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/13/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let mut data = contents.trim().lines();
    let ts = data.next().unwrap().parse::<usize>().unwrap();
    let raw = data.next().unwrap().split(',').map(|dig| dig.parse());
    let freqs: Vec<usize> = raw
        .clone()
        .filter(Result::is_ok)
        .map(|x| x.unwrap())
        .collect();
    println!("Star 1: {}", star_1(ts, &freqs));
    println!("Star 2: {}", star_2(raw));
}

fn star_1(ts: usize, freqs: &[usize]) -> usize {
    let (min_f, diff) = freqs
        .iter()
        .map(|f| (f, ts / f))
        .map(|(f, dts)| (f, f * (dts + 1) - ts))
        .min_by_key(|(_, diff)| *diff)
        .unwrap();
    min_f * diff
}

fn star_2(raw: impl Iterator<Item = Result<usize, std::num::ParseIntError>>) -> usize {
    let mut rem_freqs: Vec<(usize, usize)> = raw
        .enumerate()
        .filter(|(_idx, f)| f.is_ok())
        .map(|(idx, f)| (idx, f.unwrap()))
        .collect();
    rem_freqs.sort_unstable_by_key(|(_rem, f)| std::cmp::Reverse(*f));

    let (start, _) = rem_freqs
        .iter()
        .fold((1, 1), |(start, period_time), (rem, freq)| {
            let start = find_period_time(start, period_time, *rem, *freq);
            // Freq's are all prime, but I mean they could be not prime.
            let period_time = lcm(period_time, *freq);
            (start, period_time)
        });
    start
}

fn check_rems(ts: usize, rem: usize, freq: usize) -> bool {
    (ts + rem) % freq == 0
}

fn find_period_time(start: usize, period_time: usize, rem: usize, freq: usize) -> usize {
    (start..)
        .step_by(period_time)
        .find(|t| check_rems(*t, rem, freq))
        .unwrap()
}

fn lcm(num_1: usize, num_2: usize) -> usize {
    let (mut num, mut den) = if num_1 > num_2 {
        (num_1, num_2)
    } else {
        (num_2, num_1)
    };
    let mut rem = num % den;
    while rem != 0 {
        num = den;
        den = rem;
        rem = num % den;
    }
    let gcd = den;
    (num_1 * num_2) / gcd
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let file = "input/13/test";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let mut data = contents.trim().lines();
        data.next().unwrap();
        let raw = data.next().unwrap().split(',').map(|dig| dig.parse());
        assert_eq!(1068781, star_2(raw));
    }

    #[test]
    fn test_1() {
        let file = "input/13/test_1";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let mut data = contents.trim().lines();
        data.next().unwrap();
        let raw = data.next().unwrap().split(',').map(|dig| dig.parse());
        assert_eq!(754018, star_2(raw));
    }

    #[test]
    fn test_2() {
        let file = "input/13/test_2";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let mut data = contents.trim().lines();
        data.next().unwrap();
        let raw = data.next().unwrap().split(',').map(|dig| dig.parse());
        assert_eq!(779210, star_2(raw));
    }

    #[test]
    fn test_3() {
        let file = "input/13/test_3";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let mut data = contents.trim().lines();
        data.next().unwrap();
        let raw = data.next().unwrap().split(',').map(|dig| dig.parse());
        assert_eq!(1261476, star_2(raw));
    }

    #[test]
    fn test_4() {
        let file = "input/13/test_4";
        let mut file = File::open(file).expect("Opening file error");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Read to string error");
        let mut data = contents.trim().lines();
        data.next().unwrap();
        let raw = data.next().unwrap().split(',').map(|dig| dig.parse());
        assert_eq!(1202161486, star_2(raw));
    }
}
