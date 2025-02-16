enum Register {
    A,
    B,
}
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}
#[derive(Debug)]
struct State {
    a: u64,
    b: u64,
    program_counter: usize,
}
fn main() {
    let program: Vec<Instruction> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let (instruction, operands) = line
                .split_once(' ')
                .expect("Couldn't find the first space in line");
            match instruction {
                "hlf" => {
                    let register = parse_register(operands);
                    Instruction::Hlf(register)
                }
                "tpl" => {
                    let register = parse_register(operands);
                    Instruction::Tpl(register)
                }
                "inc" => {
                    let register = parse_register(operands);
                    Instruction::Inc(register)
                }
                "jmp" => {
                    let offset = operands.parse::<isize>().expect("Bad offset integer");
                    Instruction::Jmp(offset)
                }
                "jie" => {
                    let (register, offset) = operands.split_once(", ").expect(
                        "Couldn't find ', ' separator pattern in operands of jie instruction",
                    );
                    let register = parse_register(register);
                    let offset = offset.parse::<isize>().expect("Bad offset integer");
                    Instruction::Jie(register, offset)
                }
                "jio" => {
                    let (register, offset) = operands.split_once(", ").expect(
                        "Couldn't find ', ' separator pattern in operands of jio instruction",
                    );
                    let register = parse_register(register);
                    let offset = offset.parse::<isize>().expect("Bad offset integer");
                    Instruction::Jio(register, offset)
                }
                _ => panic!("Bad instruction"),
            }
        })
        .collect();
    let mut state = State {
        a: 0,
        b: 0,
        program_counter: 0,
    };
    let output = loop {
        let instruction = program.get(state.program_counter);
        if instruction.is_none() {
            break state.b;
        }
        let instruction = instruction.unwrap();
        match instruction {
            Instruction::Hlf(Register::A) => {
                state.a /= 2;
                state.program_counter += 1;
            }
            Instruction::Hlf(Register::B) => {
                state.b /= 2;
                state.program_counter += 1;
            }
            Instruction::Tpl(Register::A) => {
                state.a *= 3;
                state.program_counter += 1;
            }
            Instruction::Tpl(Register::B) => {
                state.b *= 3;
                state.program_counter += 1;
            }
            Instruction::Inc(Register::A) => {
                state.a += 1;
                state.program_counter += 1;
            }
            Instruction::Inc(Register::B) => {
                state.b += 1;
                state.program_counter += 1;
            }
            Instruction::Jmp(offset) => {
                state.program_counter = state.program_counter.wrapping_add_signed(*offset);
            }
            Instruction::Jie(Register::A, offset) if state.a % 2 == 0 => {
                state.program_counter = state.program_counter.wrapping_add_signed(*offset);
            }
            Instruction::Jie(Register::B, offset) if state.b % 2 == 0 => {
                state.program_counter = state.program_counter.wrapping_add_signed(*offset);
            }
            Instruction::Jio(Register::A, offset) if state.a == 1 => {
                state.program_counter = state.program_counter.wrapping_add_signed(*offset);
            }
            Instruction::Jio(Register::B, offset) if state.b == 1 => {
                state.program_counter = state.program_counter.wrapping_add_signed(*offset);
            }
            _ => {
                state.program_counter += 1;
            }
        }
    };
    println!("{}", output);
}

fn parse_register(operands: &str) -> Register {
    match operands {
        "a" => Register::A,
        "b" => Register::B,
        _ => panic!("Invalid register name"),
    }
}
