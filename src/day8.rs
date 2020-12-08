use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    ACC(i32),
    NOP(i32),
    JMP(i32),
}

#[derive(Debug, Clone)]
enum ParseError {
    MalformedInput(String),
    ArgumentParseError(ParseIntError),
}

fn invalid_input(input: &str) -> Result<Instruction, ParseError> {
    Err(ParseError::MalformedInput(input.to_owned()))
}

impl Instruction {
    fn parse(line: &str) -> Result<Instruction, ParseError> {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        if parts.len() != 2 {
            return invalid_input(line);
        }

        let value = parts[1].parse::<i32>().map_err(ParseError::ArgumentParseError)?;
        match parts[0] {
            "acc" => Ok(Instruction::ACC(value)),
            "nop" => Ok(Instruction::NOP(value)),
            "jmp" => Ok(Instruction::JMP(value)),
            _ => invalid_input(line),
        }
    }
}

#[derive(Clone)]
struct Cpu {
    acc: i32,
    ptr: i32,
}

impl Cpu {
    pub fn execute(&mut self, ins: &Instruction) {
        match ins {
            Instruction::ACC(v) => self.acc += v,
            Instruction::JMP(j) => {
                self.ptr += j;
                return;
            },
            Instruction::NOP(_) => (),
        }
    
        self.ptr += 1; // by default move one
    }

    fn u_ptr(&self) -> Option<usize> {
        if self.ptr >= 0 {
            Some(self.ptr as usize)
        }
        else {
            None
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| Instruction::parse(line).unwrap())
        .collect()
}

enum ExecuteResult {
    Looped(i32),
    Completed(i32),
    Failed,
}

fn run_cpu(program: &[Instruction], cpu: &mut Cpu, instruction_record: &mut [bool]) -> ExecuteResult {
    while let Some(u_ptr) = cpu.u_ptr() {
        if u_ptr == program.len() {
            return ExecuteResult::Completed(cpu.acc)
        }
        
        if instruction_record[u_ptr] {
            return ExecuteResult::Looped(cpu.acc)
        }

        instruction_record[u_ptr] = true;

        cpu.execute(&program[u_ptr]);
    }

    ExecuteResult::Failed
}

#[aoc(day8, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> i32 {
    let mut cpu = Cpu { acc: 0, ptr: 0 };

    let mut instruction_record = vec![false; instructions.len()];

    match run_cpu(instructions, &mut cpu, &mut instruction_record) {
        ExecuteResult::Looped(v) | ExecuteResult::Completed(v) => v,
        ExecuteResult::Failed => panic!("Failed to execute?"),
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> i32 {
    let cpu = Cpu { acc: 0, ptr: 0 };

    let instruction_record = vec![false; instructions.len()];

    fn descend(program: &[Instruction], instruction_record: &[bool], cpu: &Cpu, can_mutate: bool) -> Option<i32> {
        let u_ptr = cpu.u_ptr()?;

        if u_ptr == program.len() {
            // exit condition
            return Some(cpu.acc);
        }
    
        if instruction_record[u_ptr] {
            return None;
        }

        let ins = &program[u_ptr];

        // if we already mutated the code in some way, we can't mutate anymore so just execute the program going forwards.
        if !can_mutate {
            return match run_cpu(program, &mut cpu.clone(), &mut instruction_record.to_vec()) {
                ExecuteResult::Completed(v) => Some(v),
                _ => None,
            }
        }

        let mut instruction_record = instruction_record.to_vec();
        instruction_record[u_ptr] = true;

        let mutated = match ins {
            Instruction::ACC(_) => vec![(can_mutate, program.to_vec())],
            Instruction::JMP(v) | Instruction::NOP(v) => {
                // create two parallel executions w/ the instruction swapped for one
                fn mutate_program(program: &[Instruction], loc: usize, ins: Instruction) -> Vec<Instruction> {
                    let mut program = program.to_vec();
                    program[loc] = ins;
                    program
                }

                let with_jmp = mutate_program(program, u_ptr, Instruction::JMP(*v));
                let with_nop = mutate_program(program, u_ptr, Instruction::NOP(*v));

                vec![
                    (*ins == Instruction::JMP(*v), with_jmp),
                    (*ins == Instruction::NOP(*v), with_nop),
                ]
            },
        };
        
        mutated.into_iter()
            .map(|(can_mutate, program)| {
                let mut cpu = cpu.clone();
                cpu.execute(&program[cpu.u_ptr()?]);

                descend(&program, &instruction_record, &cpu, can_mutate)
            })
            .flatten()
            .next()
    }

    descend(instructions, &instruction_record, &cpu, true).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_given_input_part1() {
        let input = "nop +0\n\
            acc +1\n\
            jmp +4\n\
            acc +3\n\
            jmp -3\n\
            acc -99\n\
            acc +1\n\
            jmp -4\n\
            acc +6";

        let instructions = input_generator(input);

        assert_eq!(
            instructions,
            vec![
                Instruction::NOP(0),
                Instruction::ACC(1),
                Instruction::JMP(4),
                Instruction::ACC(3),
                Instruction::JMP(-3),
                Instruction::ACC(-99),
                Instruction::ACC(1),
                Instruction::JMP(-4),
                Instruction::ACC(6),
            ],
        );
    }

    #[test]
    fn given_input_part1() {
        let input = "nop +0\n\
            acc +1\n\
            jmp +4\n\
            acc +3\n\
            jmp -3\n\
            acc -99\n\
            acc +1\n\
            jmp -4\n\
            acc +6";

        let instructions = input_generator(input);
        for (i, ins) in instructions.iter().enumerate() {
            println!("{:>3} {:?}", i, ins);
        }

        let acc_value = solve_part1(&instructions);

        assert_eq!(5, acc_value); // provided
    }

    #[test]
    fn given_input_part2() {
        let input = "nop +0\n\
                    acc +1\n\
                    jmp +4\n\
                    acc +3\n\
                    jmp -3\n\
                    acc -99\n\
                    acc +1\n\
                    jmp -4\n\
                    acc +6";

        let program = input_generator(input);

        let acc_value = solve_part2(&program);

        assert_eq!(8, acc_value); // provided
    }
}