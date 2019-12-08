use std::collections::{HashMap, VecDeque};

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

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Sum(LongRegisters),
    Multiply(LongRegisters),
    Input(ShortRegister),
    Output(LongRegisters),
    JumpIfTrue(LongRegisters),
    JumpIfFalse(LongRegisters),
    LessThan(LongRegisters),
    Equals(LongRegisters),
    Stop,
}

impl Operation {
    fn from_opcode(opcode: Opcode, code: &[i32], address: usize) -> Self {
        match opcode {
            Opcode::Sum(OpcodeData {
                noun_mode,
                verb_mode,
                output_mode,
            }) => {
                let noun = code[address + 1];
                let verb = code[address + 2];
                let output = code[address + 3];
                let noun_val = match noun_mode {
                    ParameterMode::Position => code[noun as usize],
                    ParameterMode::Immediate => noun,
                };
                let verb_val = match verb_mode {
                    ParameterMode::Position => code[verb as usize],
                    ParameterMode::Immediate => verb,
                };
                let output_val = match output_mode {
                    ParameterMode::Position => output,
                    ParameterMode::Immediate => panic!("Output in invalid mode"),
                };
                Operation::Sum(LongRegisters {
                    noun: noun_val,
                    verb: verb_val,
                    output: output_val,
                })
            }
            Opcode::Multiply(OpcodeData {
                noun_mode,
                verb_mode,
                output_mode,
            }) => {
                let noun = code[address + 1];
                let verb = code[address + 2];
                let output = code[address + 3];
                let noun_val = match noun_mode {
                    ParameterMode::Position => code[noun as usize],
                    ParameterMode::Immediate => noun,
                };
                let verb_val = match verb_mode {
                    ParameterMode::Position => code[verb as usize],
                    ParameterMode::Immediate => verb,
                };
                let output_val = match output_mode {
                    ParameterMode::Position => output,
                    ParameterMode::Immediate => panic!("Output in invalid mode"),
                };
                Operation::Multiply(LongRegisters {
                    noun: noun_val,
                    verb: verb_val,
                    output: output_val,
                })
            }
            Opcode::Input => {
                let register = code[address + 1];
                Operation::Input(ShortRegister(register))
            }
            Opcode::Output(OpcodeData {
                noun_mode,
                verb_mode: _,
                output_mode: _,
            }) => {
                let noun = code[address + 1];
                let noun_val = match noun_mode {
                    ParameterMode::Position => code[noun as usize],
                    ParameterMode::Immediate => noun,
                };
                Operation::Output(LongRegisters {
                    noun: noun_val,
                    verb: 0,
                    output: 0,
                })
            }
            Opcode::JumpIfTrue(OpcodeData {
                noun_mode,
                verb_mode,
                output_mode: _,
            }) => {
                let noun = code[address + 1];
                let verb = code[address + 2];
                let noun_val = match noun_mode {
                    ParameterMode::Position => code[noun as usize],
                    ParameterMode::Immediate => noun,
                };
                let verb_val = match verb_mode {
                    ParameterMode::Position => code[verb as usize],
                    ParameterMode::Immediate => verb,
                };
                Operation::JumpIfTrue(LongRegisters {
                    noun: noun_val,
                    verb: verb_val,
                    output: -1,
                })
            }
            Opcode::JumpIfFalse(OpcodeData {
                noun_mode,
                verb_mode,
                output_mode: _,
            }) => {
                let noun = code[address + 1];
                let verb = code[address + 2];
                let noun_val = match noun_mode {
                    ParameterMode::Position => code[noun as usize],
                    ParameterMode::Immediate => noun,
                };
                let verb_val = match verb_mode {
                    ParameterMode::Position => code[verb as usize],
                    ParameterMode::Immediate => verb,
                };
                Operation::JumpIfFalse(LongRegisters {
                    noun: noun_val,
                    verb: verb_val,
                    output: -1,
                })
            }
            Opcode::Equals(OpcodeData {
                noun_mode,
                verb_mode,
                output_mode,
            }) => {
                let noun = code[address + 1];
                let verb = code[address + 2];
                let output = code[address + 3];
                let noun_val = match noun_mode {
                    ParameterMode::Position => code[noun as usize],
                    ParameterMode::Immediate => noun,
                };
                let verb_val = match verb_mode {
                    ParameterMode::Position => code[verb as usize],
                    ParameterMode::Immediate => verb,
                };
                let output_val = match output_mode {
                    ParameterMode::Position => output,
                    ParameterMode::Immediate => panic!("Output in invalid mode"),
                };
                Operation::Equals(LongRegisters {
                    noun: noun_val,
                    verb: verb_val,
                    output: output_val,
                })
            }
            Opcode::LessThan(OpcodeData {
                noun_mode,
                verb_mode,
                output_mode,
            }) => {
                let noun = code[address + 1];
                let verb = code[address + 2];
                let output = code[address + 3];
                let noun_val = match noun_mode {
                    ParameterMode::Position => code[noun as usize],
                    ParameterMode::Immediate => noun,
                };
                let verb_val = match verb_mode {
                    ParameterMode::Position => code[verb as usize],
                    ParameterMode::Immediate => verb,
                };
                let output_val = match output_mode {
                    ParameterMode::Position => output,
                    ParameterMode::Immediate => panic!("Output in invalid mode"),
                };
                Operation::LessThan(LongRegisters {
                    noun: noun_val,
                    verb: verb_val,
                    output: output_val,
                })
            }
            Opcode::Stop => Operation::Stop,
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Sum(OpcodeData),
    Multiply(OpcodeData),
    Input,
    Output(OpcodeData),
    JumpIfTrue(OpcodeData),
    JumpIfFalse(OpcodeData),
    LessThan(OpcodeData),
    Equals(OpcodeData),
    Stop,
}

#[derive(Debug)]
struct OpcodeData {
    noun_mode: ParameterMode,
    verb_mode: ParameterMode,
    output_mode: ParameterMode,
}

impl Opcode {
    fn from_int(opcode: i32) -> Self {
        let opcode_string = format!("{:05}", opcode);
        let (modes, opcode) = opcode_string.split_at(3);
        let modes: Vec<char> = modes.chars().collect();
        let output_mode =
            ParameterMode::from(modes[0].to_digit(10).expect("Could not parse mode") as u8);
        let verb_mode =
            ParameterMode::from(modes[1].to_digit(10).expect("Could not parse mode") as u8);
        let noun_mode =
            ParameterMode::from(modes[2].to_digit(10).expect("Could not parse mode") as u8);

        let opcode_data = OpcodeData {
            noun_mode,
            verb_mode,
            output_mode,
        };
        match opcode {
            "01" => Opcode::Sum(opcode_data),
            "02" => Opcode::Multiply(opcode_data),
            "03" => Opcode::Input,
            "04" => Opcode::Output(opcode_data),
            "05" => Opcode::JumpIfTrue(opcode_data),
            "06" => Opcode::JumpIfFalse(opcode_data),
            "07" => Opcode::LessThan(opcode_data),
            "08" => Opcode::Equals(opcode_data),
            "99" => Opcode::Stop,
            &_ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct LongRegisters {
    noun: i32,
    verb: i32,
    output: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct ShortRegister(i32);

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from(mode: u8) -> Self {
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid param mode"),
        }
    }
}

fn preprocess_intcode(code: &Vec<i32>, noun: i32, verb: i32) -> Vec<i32> {
    let mut new_code = code.clone();
    new_code[1] = noun;
    new_code[2] = verb;
    new_code
}

fn update_code(
    code: &mut [i32],
    operation: Operation,
    address: usize,
    input: &mut VecDeque<i32>,
) -> (usize, Option<i32>) {
    match operation {
        Operation::Sum(LongRegisters { noun, verb, output }) => {
            code[output as usize] = noun + verb;
            (address + 4, None)
        }
        Operation::Multiply(LongRegisters { noun, verb, output }) => {
            code[output as usize] = noun * verb;
            (address + 4, None)
        }
        Operation::Input(ShortRegister(pointer)) => {
            code[pointer as usize] = input.pop_front().expect("Could not pop");
            (address + 2, None)
        }
        Operation::Output(LongRegisters {
            noun,
            verb: _,
            output: _,
        }) => (address + 2, Some(noun)),
        Operation::JumpIfTrue(LongRegisters {
            noun,
            verb,
            output: _,
        }) => {
            if noun != 0 {
                (verb as usize, None)
            } else {
                (address + 3, None)
            }
        }
        Operation::JumpIfFalse(LongRegisters {
            noun,
            verb,
            output: _,
        }) => {
            if noun == 0 {
                (verb as usize, None)
            } else {
                (address + 3, None)
            }
        }
        Operation::LessThan(LongRegisters { noun, verb, output }) => {
            code[output as usize] = (noun < verb) as i32;
            (address + 4, None)
        }
        Operation::Equals(LongRegisters { noun, verb, output }) => {
            code[output as usize] = (noun == verb) as i32;
            (address + 4, None)
        }
        Operation::Stop => (address, None),
    }
}

fn parse_intcode_old(
    address: usize,
    mut code: Vec<i32>,
    input: &mut VecDeque<i32>,
    output: Option<i32>,
) -> Option<i32> {
    //println!("Code: {:?}", code);
    //println!("Address: {}", address);
    if address + 1 >= code.len() {
        return output;
    }
    let opcode = Opcode::from_int(code[address]);
    //println!("{:?}", opcode);
    let operation = Operation::from_opcode(opcode, &code, address);
    //println!("Operation: {:?}", operation);
    match operation {
        Operation::Stop => output,
        _ => {
            let (address, output) = update_code(&mut code, operation, address, input);
            parse_intcode_old(address, code, input, output)
        }
    }
}

fn chain_output(intcode: &Vec<i32>, control_vec: &Vec<i32>) -> i32 {
    let mut input = 0;
    let input_queue = &mut VecDeque::new();
    for control in control_vec {
        input_queue.push_back(*control);
        input_queue.push_back(input);
        input = parse_intcode(0, &mut intcode.clone(), input_queue, None)
            .0
            .expect("No output");
    }
    input
}

struct Amplifier<'a> {
    id: char,
    address: usize,
    input: &'a mut VecDeque<i32>,
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

fn parse_intcode(
    address: usize,
    code: &mut Vec<i32>,
    input: &mut VecDeque<i32>,
    output: Option<i32>,
) -> (Option<i32>, usize) {
    //println!("Code: {:?}", code);
    //println!("Address: {}", address);
    if address + 1 >= code.len() {
        return (output, 0);
    }
    let opcode = Opcode::from_int(code[address]);
    //println!("{:?}", opcode);
    let operation = Operation::from_opcode(opcode, &code, address);
    //println!("Operation: {:?}", operation);
    match operation {
        Operation::Stop => (output, 0),
        _ => {
            let (address, output) = update_code(code, operation, address, input);
            if let Some(output) = output {
                return (Some(output), address);
            }
            parse_intcode(address, code, input, output)
        }
    }
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
                            let tmp_result = chain_output_feedback(&intcode, &control);
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
    println!("{:?}, {}", best_control, result);
}

//fn day_5() {
//    println!("Star 2");
//    let program: Vec<i32> = INTCODE_DAY5.iter().cloned().collect();
//    let input_vec = vec![1; 10];
//    let result = parse_intcode(0, program, input_vec, None);
//    println!("{:?}", result);
//}

fn chain_output_feedback(intcode: &Vec<i32>, control_vec: &Vec<i32>) -> i32 {
    //println!("Chain output feedback: {:?}", control_vec);
    let input = Some(0);
    let mut output = input;
    let mut amplifiers: Vec<Amplifier> = Vec::new();
    let queue = &mut VecDeque::new();
    let code = &mut intcode.clone();
    amplifiers.push(Amplifier {
        id: 'A',
        address: 0,
        input: queue,
        code: code,
    });
    let queue = &mut VecDeque::new();
    let code = &mut intcode.clone();
    amplifiers.push(Amplifier {
        id: 'B',
        address: 0,
        input: queue,
        code: code,
    });
    let queue = &mut VecDeque::new();
    let code = &mut intcode.clone();
    amplifiers.push(Amplifier {
        id: 'C',
        address: 0,
        input: queue,
        code: code,
    });
    let queue = &mut VecDeque::new();
    let code = &mut intcode.clone();
    amplifiers.push(Amplifier {
        id: 'D',
        address: 0,
        input: queue,
        code: code,
    });
    let queue = &mut VecDeque::new();
    let code = &mut intcode.clone();
    amplifiers.push(Amplifier {
        id: 'E',
        address: 0,
        input: queue,
        code: code,
    });

    let mut last_some_output = -1;
    while output.is_some() {
        for (counter, amplifier) in &mut amplifiers.iter_mut().enumerate() {
            println!("{} from address: {}", amplifier.id, amplifier.address);
            amplifier.input.push_back(control_vec[counter]);
            amplifier
                .input
                .push_back(output.expect("Could not push first"));
            let output_tuple = parse_intcode(
                amplifier.address,
                &mut amplifier.code,
                amplifier.input,
                None,
            );
            output = output_tuple.0;
            amplifier.address = output_tuple.1;
            if output.is_none() {
                return last_some_output;
            }
        }
        if let Some(output) = output {
            last_some_output = output;
        };
    }
    last_some_output
}

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
                            let tmp_result = chain_output_feedback(&intcode, &control);
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
    let intcode = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    println!("{:?}", intcode);
    let control = vec![9, 8, 7, 6, 5];
    let output = chain_output_feedback(&intcode, &control);
    println!("{}", output);
    //star_1();
    //star_2();
}

#[cfg(test)]
mod tests_7 {
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
