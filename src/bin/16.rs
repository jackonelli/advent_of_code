use std::fs::File;
use std::io::prelude::*;

const BASE_PATTERN: &'static [i32] = &[0, 1, 0, -1];

fn first_nth_as_int(phase: &[i32], n: usize) -> i32 {
    assert!(phase.len() >= n);
    phase.iter().take(n).enumerate().fold(0, |acc, (ind, x)| {
        acc + x * 10_i32.pow((n - (ind + 1)) as u32)
    })
}

fn input_to_array(input_path: &str) -> Vec<i32> {
    let file = input_path;
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");

    contents.trim().chars().fold(Vec::new(), |mut acc, char| {
        acc.push(char.to_digit(10).expect("Digit fail") as i32);
        acc
    })
}

fn generate_pattern(repeat: usize) -> Vec<i32> {
    let pattern = BASE_PATTERN.iter().fold(Vec::new(), |mut acc, x| {
        acc.extend(vec![x; repeat]);
        acc
    });
    pattern
}

fn convolve(input: &[i32], pattern: &[i32]) -> i32 {
    pattern
        .iter()
        .cycle()
        .skip(1)
        .zip(input.iter())
        .fold(0, |acc, (x, y)| acc + x * y)
}

fn positive_single_digit(number: i32) -> i32 {
    let n = number.abs();
    n % 10
}

fn gen_new_phase(old_phase: &[i32]) -> Vec<i32> {
    (0..old_phase.len()).fold(Vec::new(), |mut acc, ind| {
        let pattern = generate_pattern(ind + 1);
        acc.push(positive_single_digit(convolve(old_phase, &pattern)));
        acc
    })
}

fn main() {
    // Correct 58100105
    env_logger::init();
    let input = input_to_array("input/16/input");
    let num_phases = 100;
    let phase = input;
    let end_phase = (0..num_phases).fold(phase, |acc, iter| {
        println!("---- {} -----", iter);
        gen_new_phase(&acc)
    });
    println!("{}", first_nth_as_int(&end_phase, 8));
}

#[cfg(test)]
mod tests_16 {
    use super::*;
    #[test]
    fn ex_1() {
        let input = input_to_array("input/16/ex1");
        let num_phases = 4;
        let mut phase = input;
        let truth = vec![48226158, 34040438, 03415518, 01029498];
        for iter in 0..num_phases {
            phase = gen_new_phase(&phase);
            assert_eq!(truth[iter], first_nth_as_int(&phase, 8));
        }
    }
    #[test]
    fn ex_2() {
        let input = input_to_array("input/16/ex2");
        let num_phases = 100;
        let mut phase = input;
        let truth = 24176176;
        for _iter in 0..num_phases {
            phase = gen_new_phase(&phase);
        }
        assert_eq!(truth, first_nth_as_int(&phase, 8));
    }
    #[test]
    fn test_first_digit() {
        assert_eq!(1, positive_single_digit(1));
        assert_eq!(1, positive_single_digit(-1));
        assert_eq!(0, positive_single_digit(10));
        assert_eq!(0, positive_single_digit(100));
        assert_eq!(9, positive_single_digit(29));
        assert_eq!(7, positive_single_digit(-202727));
    }
}
