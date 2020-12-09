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

#[aoc_generator(day9)]
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

#[aoc(day9, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> i32 {
    let mut cpu = Cpu { acc: 0, ptr: 0 };

    let mut instruction_record = vec![false; instructions.len()];

    match run_cpu(instructions, &mut cpu, &mut instruction_record) {
        ExecuteResult::Looped(v) | ExecuteResult::Completed(v) => v,
        ExecuteResult::Failed => panic!("Failed to execute?"),
    }
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
}