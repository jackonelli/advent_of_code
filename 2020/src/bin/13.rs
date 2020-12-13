#![feature(test)]
#![feature(split_inclusive)]
use std::fs::File;
use std::io::prelude::*;

//749468541208439
//219279645705120
fn main() {
    let file = "input/13/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let mut data = contents.trim().lines();
    let ts = data.next().unwrap().parse::<usize>().unwrap();
    let f = 59;
    let raw = data.next().unwrap().split(',').map(|dig| dig.parse());
    let freqs: Vec<usize> = raw
        .clone()
        .filter(Result::is_ok)
        .map(|x| x.unwrap())
        .collect();
    //println!("Star 1: {}", star_1(ts, &freqs));
    println!("Star 2: {}", star_2(raw));
}

fn check_rems(ts: usize, rem: usize, freq: usize) -> bool {
    (ts + rem) % freq == 0
}

fn star_2(raw: impl Iterator<Item = Result<usize, std::num::ParseIntError>>) -> usize {
    let mut rem_freqs: Vec<(usize, usize)> = raw
        .enumerate()
        .filter(|(_idx, f)| f.is_ok())
        .map(|(idx, f)| (idx, f.unwrap()))
        .collect();
    for (rem, f) in &rem_freqs {
        println!("{}, {:?}", f, rem);
    }
    //let (start, period_time) = find_period_time(1, 1, 61, 17);

    rem_freqs.sort_unstable_by_key(|(_rem, f)| std::cmp::Reverse(*f));
    //rem_freqs.sort_unstable_by_key(|(rem, f)| *f);

    let (rem, freq) = rem_freqs[0];
    let (mut start, mut period_time) = find_period_time(1, 1, rem, freq);
    println!("start: {}, period_time: {}", start, period_time);
    for (rem, freq) in &rem_freqs[1..3] {
        let res = find_period_time(start, period_time, *rem, *freq);

        start = res.0;
        period_time = res.1;
        println!("start: {}, period_time: {}", start, period_time);
        println!("*****");
    }
    star_2_semi(start, period_time, &rem_freqs[3..])
}

fn star_2_semi(start: usize, period_time: usize, rem_freqs: &[(usize, usize)]) -> usize {
    let mut i = start;
    while i < 749468541208439 {
        if check_all_rems(i, rem_freqs) {
            return i;
        };
        i += period_time;
    }
    panic!();
}

fn check_all_rems(ts: usize, rem_freqs: &[(usize, usize)]) -> bool {
    let mut ok = true;
    for (rem, freq) in rem_freqs {
        ok &= check_rems(ts, *rem, *freq);
    }
    ok
}

fn find_period_time(start: usize, period_time: usize, rem: usize, freq: usize) -> (usize, usize) {
    println!(
        "Checking: {}, {}. Start: {}, Period: {}",
        freq, rem, start, period_time
    );
    let mut i = start;
    let mut new_start = None;
    while i < 749468541208439 {
        if check_rems(i, rem, freq) {
            match new_start {
                Some(new_start) => {
                    let new_period_time = i - new_start;
                    return (new_start, new_period_time);
                }
                None => new_start = Some(i),
            }
        };
        i += period_time;
    }
    panic!();
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
