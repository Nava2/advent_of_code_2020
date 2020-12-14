use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Mask {
    and_mask: u64,
    or_mask: u64,
}

impl Mask {
    fn parse(mask: &str) -> Option<Mask> {
        let mut and_mask = 0;
        let mut or_mask = 0;
        for c in mask.chars() {
            match c {
                '1' => { 
                    or_mask |= 1; 
                    and_mask |= 1; 
                },
                '0' => { 
                    or_mask |= 0; 
                    and_mask |= 0; 
                },
                'X' => {
                    or_mask |= 0;
                    and_mask |= 1;
                }
                _ => return None,
            };

            or_mask <<= 1;
            and_mask <<= 1;
        }

        or_mask >>= 1;
        and_mask >>= 1;

        Some(
            Mask {
                and_mask,
                or_mask,
            }
        )
    }

    fn mask(&self, value: u64) -> u64 {
        (value & self.and_mask) | self.or_mask
    }
}

trait Maskable {
    fn mask(&self, mask: &Option<&Mask>) -> u64;
}

impl Maskable for u64 {
    fn mask(&self, mask: &Option<&Mask>) -> u64 {
        if let Some(mask) = mask {
            mask.mask(*self)
        }
        else {
            *self
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Mask(Mask),
    Write(usize, u64),
}

impl Instruction {
    fn parse(line: &str) -> Option<Instruction> {
        lazy_static! {
            static ref MEM_RE: Regex = Regex::new(r"\s*mem\[(?P<address>\d+)\]\s*=\s*(?P<value>\d+)\s*").unwrap();
        }

        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"\s*mask\s*=\s*(?P<mask>[01X]+)\s*").unwrap();
        }

        let result = if let Some(cp) = MEM_RE.captures(line) {
            Instruction::Write(
                cp.name("address")?.as_str().parse::<usize>().ok()?,
                cp.name("value")?.as_str().parse::<u64>().ok()?,
            )
        }
        else if let Some(cp) = MASK_RE.captures(line) {
            let mask = cp.name("mask")?.as_str();
            Instruction::Mask(Mask::parse(mask)?)
        }
        else {
            panic!()
        };

        Some(result)
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| Instruction::parse(line))
        .flatten()
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(program: &[Instruction]) -> u64 {
    let mut memory = HashMap::<usize, u64>::new();
    let mut mask: Option<&Mask> = None;

    for ins in program {
        match ins {
            Instruction::Mask(m) => { mask = Some(m); },
            Instruction::Write(address, value) => { 
                let entry = memory.entry(*address).or_insert(0);
                *entry = value.mask(&mask); 
            },
        }
    }

    memory.values().fold(0u64, |a, v| a + *v)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT1: &str = 
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
         mem[8] = 11\n\
         mem[7] = 101\n\
         mem[8] = 0";

    #[test]
    fn solve_part1_given1() {
        let program = input_generator(GIVEN_INPUT1);

        assert_eq!(solve_part1(&program), 165);
    }

    #[test]
    fn input_generator_given1() {
        let program = input_generator(GIVEN_INPUT1);

        assert_eq!(
            program,
            vec![
                Instruction::Mask(
                    Mask { 
                        // mask =   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                        and_mask: 0b111111111111111111111111111111111101,
                        or_mask:                               0b1000000,
                    }
                ),
                Instruction::Write(8, 11),
                Instruction::Write(7, 101),
                Instruction::Write(8, 0),
            ]
        )
    }
}