const INTCODE_DAY7: [i32; 515] = [
    3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 59, 84, 97, 110, 191, 272, 353, 434, 99999, 3, 9,
    1002, 9, 2, 9, 101, 4, 9, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 102, 5, 9, 9, 1001, 9, 3, 9, 1002,
    9, 5, 9, 101, 5, 9, 9, 4, 9, 99, 3, 9, 102, 5, 9, 9, 101, 5, 9, 9, 1002, 9, 3, 9, 101, 2, 9, 9,
    1002, 9, 4, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9, 102, 5, 9, 9, 1001,
    9, 3, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
    9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001,
    9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9,
    99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101,
    1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9,
    3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9,
    1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9,
    4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
    1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 101, 2, 9,
    9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
    101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4,
    9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
    102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
    9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001,
    9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99,
];

#[derive(Debug)]
enum Opcode {
    Sum,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Stop,
}

impl Opcode {
    fn from_str(opcode: &str) -> Self {
        match opcode {
            "01" => Opcode::Sum,
            "02" => Opcode::Multiply,
            "03" => Opcode::Input,
            "04" => Opcode::Output,
            "05" => Opcode::JumpIfTrue,
            "06" => Opcode::JumpIfFalse,
            "07" => Opcode::LessThan,
            "08" => Opcode::Equals,
            "99" => Opcode::Stop,
            &_ => panic!("Invalid opcode"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from(mode: u32) -> Self {
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid param mode"),
        }
    }
}

fn fetch_register_value(intcode: &[i32], value: i32, mode: ParameterMode) -> Option<i32> {
    match mode {
        ParameterMode::Position => {
            let value = value as usize;
            if value < intcode.len() {
                Some(intcode[value])
            } else {
                None
            }
        }
        ParameterMode::Immediate => Some(value),
    }
}

fn get_opcode_and_mode(opcode: i32) -> (Opcode, ParameterMode, ParameterMode) {
    let opcode_string = format!("{:05}", opcode);
    let (modes, opcode) = opcode_string.split_at(3);

    let modes: Vec<char> = modes.chars().collect();
    let param_a = ParameterMode::from(modes[0].to_digit(10).expect("Could not parse mode"));
    if param_a != ParameterMode::Position {
        panic!("Param A not in position mode");
    }
    let param_b = ParameterMode::from(modes[1].to_digit(10).expect("Could not parse mode"));
    let param_c = ParameterMode::from(modes[2].to_digit(10).expect("Could not parse mode"));

    let opcode = Opcode::from_str(opcode);
    (opcode, param_b, param_c)
}

fn parse_intcode(
    intcode: &mut [i32],
    mut address: usize,
    phase: i32,
    input: i32,
) -> (Option<i32>, usize) {
    let mut phase_read = false;
    let mut instruction_length;
    while address < intcode.len() {
        //ABCDE
        //println!("Intcode: {:?}", intcode[address]);
        let (opcode, mode_b, mode_c) = get_opcode_and_mode(intcode[address]);
        let register_value_b = fetch_register_value(&intcode, intcode[address + 2], mode_b);
        let register_value_c = fetch_register_value(&intcode, intcode[address + 1], mode_c);

        match opcode {
            Opcode::Sum => {
                instruction_length = 3;
                let result_position = intcode[address + instruction_length] as usize;
                intcode[result_position] = register_value_c.unwrap() + register_value_b.unwrap();
            }
            Opcode::Multiply => {
                instruction_length = 3;
                let result_position = intcode[address + instruction_length] as usize;
                intcode[result_position] = register_value_c.unwrap() * register_value_b.unwrap();
            }
            Opcode::Input => {
                instruction_length = 1;
                let result_position = intcode[address + instruction_length] as usize;
                let value = if !phase_read {
                    phase_read = true;
                    phase
                } else {
                    input
                };
                intcode[result_position] = value;
            }
            Opcode::Output => {
                instruction_length = 1;
                let result_position = intcode[address + instruction_length] as usize;
                return (
                    Some(intcode[result_position]),
                    address + instruction_length as usize,
                );
            }
            Opcode::JumpIfTrue => {
                instruction_length = 1;
                if register_value_c.unwrap() != 0 {
                    address = register_value_b.unwrap() as usize;
                }
            }
            Opcode::JumpIfFalse => {
                instruction_length = 1;
                if register_value_c.unwrap() == 0 {
                    address = register_value_b.unwrap() as usize;
                }
            }
            Opcode::LessThan => {
                instruction_length = 3;
                let result_position = intcode[address + instruction_length] as usize;
                intcode[result_position] = if register_value_c.unwrap() < register_value_b.unwrap()
                {
                    1
                } else {
                    0
                };
            }
            Opcode::Equals => {
                instruction_length = 3;
                let result_position = intcode[address + instruction_length] as usize;
                intcode[result_position] = if register_value_c == register_value_b {
                    1
                } else {
                    0
                };
            }
            Opcode::Stop => return (None, address),
        }
        address += instruction_length + 1;
    }
    (None, address)
}

fn chain_output(intcode: &Vec<i32>, control_vec: &Vec<i32>) -> i32 {
    let mut input = 0;
    for control in control_vec {
        input = parse_intcode(&mut intcode.clone(), 0, *control, input)
            .0
            .expect("No output");
    }
    panic!();
    input
}

struct Amplifier<'a> {
    id: char,
    address: usize,
    control: i32,
    code: &'a mut Vec<i32>,
}

fn validate_phase_setting(phase_setting: &Vec<i32>, offset: usize) -> bool {
    let mut counter = vec![0; 5];
    for phase in phase_setting {
        counter[*phase as usize - offset] += 1;
        if counter[*phase as usize - offset] > 1 {
            return false;
        }
    }
    true
}

fn star_1() {
    println!("Star 1");
    let intcode = INTCODE_DAY7.iter().cloned().collect();
    let mut result = 0;
    let mut best_control = Vec::new();
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        let control = vec![a, b, c, d, e];
                        if validate_phase_setting(&control, 0) {
                            let tmp_result = chain_output(&intcode, &control);
                            if tmp_result > result {
                                result = tmp_result;
                                best_control = control.clone();
                            }
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }
    println!("Best control: {:?}, {}", best_control, result);
}

//fn chain_output_feedback(intcode: &Vec<i32>, control_vec: &Vec<i32>) -> i32 {
//    //println!("Chain output feedback: {:?}", control_vec);
//    let input = Some(0);
//    let mut output = input;
//    let mut amplifiers: Vec<Amplifier> = Vec::new();
//    let queue = &mut VecDeque::new();
//    let code = &mut intcode.clone();
//    amplifiers.push(Amplifier {
//        id: 'A',
//        address: 0,
//        input: queue,
//        code: code,
//    });
//    let queue = &mut VecDeque::new();
//    let code = &mut intcode.clone();
//    amplifiers.push(Amplifier {
//        id: 'B',
//        address: 0,
//        input: queue,
//        code: code,
//    });
//    let queue = &mut VecDeque::new();
//    let code = &mut intcode.clone();
//    amplifiers.push(Amplifier {
//        id: 'C',
//        address: 0,
//        input: queue,
//        code: code,
//    });
//    let queue = &mut VecDeque::new();
//    let code = &mut intcode.clone();
//    amplifiers.push(Amplifier {
//        id: 'D',
//        address: 0,
//        input: queue,
//        code: code,
//    });
//    let queue = &mut VecDeque::new();
//    let code = &mut intcode.clone();
//    amplifiers.push(Amplifier {
//        id: 'E',
//        address: 0,
//        input: queue,
//        code: code,
//    });
//
//    let mut last_some_output = -1;
//    while output.is_some() {
//        for (counter, amplifier) in &mut amplifiers.iter_mut().enumerate() {
//            println!("{} from address: {}", amplifier.id, amplifier.address);
//            amplifier.input.push_back(control_vec[counter]);
//            amplifier
//                .input
//                .push_back(output.expect("Could not push first"));
//            let output_tuple = _parse_intcode(
//                amplifier.address,
//                &mut amplifier.code,
//                amplifier.input,
//                None,
//            );
//            output = output_tuple.0;
//            amplifier.address = output_tuple.1;
//            if output.is_none() {
//                return last_some_output;
//            }
//        }
//        if let Some(output) = output {
//            last_some_output = output;
//        };
//    }
//    last_some_output
//}

fn star_2() {
    println!("Star 2");
    let intcode = INTCODE_DAY7.iter().cloned().collect();
    let mut result = 0;
    let mut best_control = Vec::new();
    for a in 5..10 {
        for b in 5..10 {
            for c in 5..10 {
                for d in 5..10 {
                    for e in 5..10 {
                        let control = vec![a, b, c, d, e];
                        if validate_phase_setting(&control, 5) {
                            //println!("Control {:?}", control);
                            let tmp_result = chain_output(&intcode, &control);
                            if tmp_result > result {
                                result = tmp_result;
                                best_control = control.clone();
                            }
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }
    println!("Best control: {:?}, {}", best_control, result);
}

fn main() {
    star_1();
    //star_2();
}

#[cfg(test)]
mod tests_7_2 {
    use super::*;
    #[test]
    fn test_code_1() {
        let intcode = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let true_result = 43210;
        let control = vec![4, 3, 2, 1, 0];
        let result = chain_output(&intcode, &control);
        assert_eq!(true_result, result);
    }
    #[test]
    fn test_code_2() {
        let intcode = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let true_result = 54321;
        let control = vec![0, 1, 2, 3, 4];
        let result = chain_output(&intcode, &control);
        assert_eq!(true_result, result);
    }
    #[test]
    fn test_code_3() {
        let intcode = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let true_result = 65210;
        let control = vec![1, 0, 4, 3, 2];
        let result = chain_output(&intcode, &control);
        assert_eq!(true_result, result);
    }
}
