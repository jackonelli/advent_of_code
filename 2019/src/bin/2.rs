const INTCODE: [u32; 117] = [
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 6, 23, 2, 13, 23, 27, 1,
    27, 13, 31, 1, 9, 31, 35, 1, 35, 9, 39, 1, 39, 5, 43, 2, 6, 43, 47, 1, 47, 6, 51, 2, 51, 9, 55,
    2, 55, 13, 59, 1, 59, 6, 63, 1, 10, 63, 67, 2, 67, 9, 71, 2, 6, 71, 75, 1, 75, 5, 79, 2, 79,
    10, 83, 1, 5, 83, 87, 2, 9, 87, 91, 1, 5, 91, 95, 2, 13, 95, 99, 1, 99, 10, 103, 1, 103, 2,
    107, 1, 107, 6, 0, 99, 2, 14, 0, 0,
];

#[derive(Debug)]
enum Opcode {
    Sum(Registers),
    Multiply(Registers),
    Stop,
}

impl Opcode {
    fn from_sequence(seq: &[u32]) -> Self {
        match seq[0] {
            1 => Opcode::Sum(Registers {
                noun: seq[1],
                verb: seq[2],
                output: seq[3],
            }),
            2 => Opcode::Multiply(Registers {
                noun: seq[1],
                verb: seq[2],
                output: seq[3],
            }),
            99 => Opcode::Stop,
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Debug)]
struct Registers {
    noun: u32,
    verb: u32,
    output: u32,
}

fn preprocess_intcode(code: &Vec<u32>, noun: u32, verb: u32) -> Vec<u32> {
    let mut new_code = code.clone();
    new_code[1] = noun;
    new_code[2] = verb;
    new_code
}

fn update_code(code: Vec<u32>, op_code: Opcode) -> Vec<u32> {
    let mut new_code = code.clone();
    match op_code {
        Opcode::Sum(Registers { noun, verb, output }) => {
            new_code[output as usize] = code[noun as usize] + code[verb as usize];
            new_code
        }
        Opcode::Multiply(Registers { noun, verb, output }) => {
            new_code[output as usize] = code[noun as usize] * code[verb as usize];
            new_code
        }
        Opcode::Stop => panic!("Here, we should not be"),
    }
}

fn parse_intcode(address: usize, code: Vec<u32>) -> Vec<u32> {
    if address + 4 >= code.len() {
        return code;
    }
    let op_code = Opcode::from_sequence(&code[address..(address + 4)]);
    match op_code {
        Opcode::Stop => code,
        _ => {
            let code = update_code(code, op_code);
            parse_intcode(address + 4, code)
        }
    }
}

fn brute_force(intcode: &Vec<u32>, target: u32, noun: u32, verb: u32) -> (u32, u32) {
    let preprocessed = preprocess_intcode(&intcode, noun, verb);
    let output = parse_intcode(0, preprocessed)[0];
    if output == target {
        (noun, verb)
    } else if noun == 99 {
        brute_force(&intcode, target, 0, verb + 1)
    } else {
        brute_force(&intcode, target, noun + 1, verb)
    }
}

fn star_1() {
    let intcode: Vec<u32> = INTCODE.iter().cloned().collect();
    let noun = 12;
    let verb = 2;
    let preprocessed = preprocess_intcode(&intcode, noun, verb);
    let code = parse_intcode(0, preprocessed);
    println!("First element in parsed code: {}", code[0]);
}

fn star_2() {
    const OUTPUT: u32 = 19_690_720;
    let intcode: Vec<u32> = INTCODE.iter().cloned().collect();
    let (noun, verb) = brute_force(&intcode, OUTPUT, 0, 74);
    println!("Input 1: {}, Input 2: {}", noun, verb);
}

fn main() {
    star_1();
    star_2();
}

#[cfg(test)]
mod tests_2 {
    use super::*;
    #[test]
    fn test_preprocess() {
        let intcode = vec![1, 0, 0, 0, 99];
        let noun = 12;
        let verb = 2;
        let true_result_code = vec![1, 12, 2, 0, 99];
        let preprocessed_code = preprocess_intcode(&intcode, noun, verb);
        assert_eq!(true_result_code, preprocessed_code);
    }

    #[test]
    fn test_code() {
        let intcode = vec![1, 0, 0, 0, 99];
        let true_result_code = vec![2, 0, 0, 0, 99];
        let result_code = parse_intcode(0, intcode);
        assert_eq!(true_result_code, result_code);

        let intcode = vec![2, 3, 0, 3, 99];
        let true_result_code = vec![2, 3, 0, 6, 99];
        let result_code = parse_intcode(0, intcode);
        assert_eq!(true_result_code, result_code);

        let intcode = vec![2, 4, 4, 5, 99, 0];
        let true_result_code = vec![2, 4, 4, 5, 99, 9801];
        let result_code = parse_intcode(0, intcode);
        assert_eq!(true_result_code, result_code);

        let intcode = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let true_result_code = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let result_code = parse_intcode(0, intcode);
        assert_eq!(true_result_code, result_code);
    }

    #[test]
    fn test_long_code() {
        let intcode = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let true_result_code = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let result_code = parse_intcode(0, intcode);
        assert_eq!(true_result_code, result_code);
    }
}
