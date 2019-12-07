const INTCODE_DAY5: [i32; 678] = [
    3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 48, 82, 225, 102, 59, 84, 224, 1001,
    224, -944, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223, 1101, 92, 58,
    224, 101, -150, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 1102,
    10, 89, 224, 101, -890, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5, 224, 1, 224, 223,
    223, 1101, 29, 16, 225, 101, 23, 110, 224, 1001, 224, -95, 224, 4, 224, 102, 8, 223, 223, 1001,
    224, 3, 224, 1, 223, 224, 223, 1102, 75, 72, 225, 1102, 51, 8, 225, 1102, 26, 16, 225, 1102, 8,
    49, 225, 1001, 122, 64, 224, 1001, 224, -113, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224,
    1, 224, 223, 223, 1102, 55, 72, 225, 1002, 174, 28, 224, 101, -896, 224, 224, 4, 224, 1002,
    223, 8, 223, 101, 4, 224, 224, 1, 224, 223, 223, 1102, 57, 32, 225, 2, 113, 117, 224, 101,
    -1326, 224, 224, 4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1, 148, 13, 224,
    101, -120, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224, 224, 1, 223, 224, 223, 4, 223, 99,
    0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999,
    1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999,
    1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225,
    1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225,
    1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 8, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 329,
    101, 1, 223, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 8,
    226, 677, 224, 102, 2, 223, 223, 1006, 224, 359, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2,
    223, 223, 1005, 224, 374, 1001, 223, 1, 223, 1108, 677, 226, 224, 1002, 223, 2, 223, 1006, 224,
    389, 101, 1, 223, 223, 107, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 404, 1001, 223, 1, 223,
    1107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 419, 1001, 223, 1, 223, 108, 677, 677, 224,
    102, 2, 223, 223, 1005, 224, 434, 1001, 223, 1, 223, 1008, 677, 226, 224, 1002, 223, 2, 223,
    1006, 224, 449, 1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 464, 1001,
    223, 1, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 479, 1001, 223, 1, 223, 1007,
    226, 226, 224, 1002, 223, 2, 223, 1005, 224, 494, 1001, 223, 1, 223, 108, 226, 226, 224, 1002,
    223, 2, 223, 1005, 224, 509, 1001, 223, 1, 223, 1007, 226, 677, 224, 1002, 223, 2, 223, 1006,
    224, 524, 101, 1, 223, 223, 1107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 539, 101, 1, 223,
    223, 1107, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 554, 1001, 223, 1, 223, 108, 677, 226,
    224, 1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1, 223, 1108, 226, 677, 224, 1002, 223, 2,
    223, 1006, 224, 584, 101, 1, 223, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 599,
    1001, 223, 1, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 614, 101, 1, 223, 223, 7,
    677, 677, 224, 1002, 223, 2, 223, 1006, 224, 629, 101, 1, 223, 223, 1008, 677, 677, 224, 102,
    2, 223, 223, 1005, 224, 644, 101, 1, 223, 223, 7, 677, 226, 224, 1002, 223, 2, 223, 1005, 224,
    659, 101, 1, 223, 223, 1108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 674, 1001, 223, 1,
    223, 4, 223, 99, 226,
];

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Sum(LongRegisters),
    Multiply(LongRegisters),
    Input(ShortRegister),
    Output(ShortRegister),
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
            Opcode::Output => {
                let register = code[address + 1];
                Operation::Output(ShortRegister(register))
            }
            Opcode::JumpIfTrue(OpcodeData {
                noun_mode,
                verb_mode,
                output_mode,
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
                output_mode,
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
    Output,
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
            "04" => Opcode::Output,
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
    input: i32,
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
            code[pointer as usize] = input;
            (address + 2, None)
        }
        Operation::Output(ShortRegister(pointer)) => (address + 2, Some(0)),
        Operation::JumpIfTrue(LongRegisters { noun, verb, output }) => {
            if noun != 0 {
                (verb as usize, None)
            } else {
                (address + 3, None)
            }
        }
        Operation::JumpIfFalse(LongRegisters { noun, verb, output }) => {
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

fn parse_intcode(address: usize, mut code: Vec<i32>, input: i32) -> Vec<i32> {
    //println!("Code: {:?}", code);
    //println!("Address: {}", address);
    if address + 1 >= code.len() {
        return code;
    }
    let opcode = Opcode::from_int(code[address]);
    //println!("{:?}", opcode);
    let operation = Operation::from_opcode(opcode, &code, address);
    //println!("Operation: {:?}", operation);
    match operation {
        Operation::Stop => code,
        _ => {
            let (address, output) = update_code(&mut code, operation, address, input);
            if let Some(output) = output {
                println!("Output {}: {}", address, output);
            };
            parse_intcode(address, code, input)
        }
    }
}

fn star_1() {
    println!("Star 1");
    let program: Vec<i32> = INTCODE_DAY5.iter().cloned().collect();
    let _ = parse_intcode(0, program, 1);
}
fn star_2() {
    println!("Star 2");
    let program: Vec<i32> = INTCODE_DAY5.iter().cloned().collect();
    let _ = parse_intcode(0, program, 5);
}

fn main() {
    star_1();
    star_2();
}

#[cfg(test)]
mod tests_5 {
    use super::*;
    const INTCODE_DAY2: [i32; 117] = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 6, 23, 2, 13, 23, 27,
        1, 27, 13, 31, 1, 9, 31, 35, 1, 35, 9, 39, 1, 39, 5, 43, 2, 6, 43, 47, 1, 47, 6, 51, 2, 51,
        9, 55, 2, 55, 13, 59, 1, 59, 6, 63, 1, 10, 63, 67, 2, 67, 9, 71, 2, 6, 71, 75, 1, 75, 5,
        79, 2, 79, 10, 83, 1, 5, 83, 87, 2, 9, 87, 91, 1, 5, 91, 95, 2, 13, 95, 99, 1, 99, 10, 103,
        1, 103, 2, 107, 1, 107, 6, 0, 99, 2, 14, 0, 0,
    ];

    fn brute_force(intcode: &Vec<i32>, target: i32, noun: i32, verb: i32) -> (i32, i32) {
        let preprocessed = preprocess_intcode(&intcode, noun, verb);
        let output = parse_intcode(0, preprocessed, -1)[0];
        if output == target {
            (noun, verb)
        } else if noun == 99 {
            brute_force(&intcode, target, 0, verb + 1)
        } else {
            brute_force(&intcode, target, noun + 1, verb)
        }
    }

    #[test]
    fn day_2_star_2() {
        const OUTPUT: i32 = 19_690_720;
        let intcode: Vec<i32> = INTCODE_DAY2.iter().cloned().collect();
        let (noun, verb) = brute_force(&intcode, OUTPUT, 0, 74);
        assert_eq!(noun, 90);
        assert_eq!(verb, 74);
    }

    #[test]
    fn day_2_star_1() {
        let intcode: Vec<i32> = INTCODE_DAY2.iter().cloned().collect();
        let noun = 12;
        let verb = 2;
        let preprocessed = preprocess_intcode(&intcode, noun, verb);
        let code = parse_intcode(0, preprocessed, -1);
        assert_eq!(code[0], 284_2648);
    }

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
        let result_code = parse_intcode(0, intcode, -1);
        assert_eq!(true_result_code, result_code);

        let intcode = vec![2, 3, 0, 3, 99];
        let true_result_code = vec![2, 3, 0, 6, 99];
        let result_code = parse_intcode(0, intcode, -1);
        assert_eq!(true_result_code, result_code);

        let intcode = vec![2, 4, 4, 5, 99, 0];
        let true_result_code = vec![2, 4, 4, 5, 99, 9801];
        let result_code = parse_intcode(0, intcode, -1);
        assert_eq!(true_result_code, result_code);

        let intcode = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let true_result_code = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let result_code = parse_intcode(0, intcode, -1);
        assert_eq!(true_result_code, result_code);
    }

    #[test]
    fn test_long_code() {
        let intcode = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let true_result_code = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let result_code = parse_intcode(0, intcode, -1);
        assert_eq!(true_result_code, result_code);
    }
}
