use regex::Regex;
use std::collections::HashMap;
use bit_vec::BitVec;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MaskBit {
    One,
    Zero,
    DontCare,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mask {
    bits: Vec<MaskBit>,
}

impl Mask {
    fn parse(mask: &str) -> Mask {
        Mask {
            bits: mask.chars().map(|c| match c {
                    '1' => MaskBit::One,
                    '0' => MaskBit::Zero,
                    'X' => MaskBit::DontCare,
                    _ => panic!(),
                })
                .collect()
        }
    }

    fn load_mask(&self) -> (u64, u64) {
        let mut and_mask = 0;
        let mut or_mask = 0;
        for c in &self.bits {
            match &c {
                MaskBit::One => { 
                    or_mask |= 1; 
                    and_mask |= 1; 
                },
                MaskBit::Zero => { 
                    or_mask |= 0; 
                    and_mask |= 0; 
                },
                MaskBit::DontCare => {
                    or_mask |= 0;
                    and_mask |= 1;
                }
            };

            or_mask <<= 1;
            and_mask <<= 1;
        }

        or_mask >>= 1;
        and_mask >>= 1;

        (and_mask, or_mask)
    }

    fn apply_mask(value: u64, mask: &(u64, u64)) -> u64 {
        let (and_mask, or_mask) = mask;
        (value & and_mask) | or_mask
    }

    fn all_addresses_iter(&self, address: &usize) -> impl Iterator<Item = usize> {
        fn _generate<'a, I>(address: &usize, bit_width: usize, bit_no: usize, mut generator: I, address_accum: &mut BitVec<u32>) -> Box<dyn Iterator<Item = usize>>
            where
                I: Iterator<Item = &'a MaskBit> + Clone,
        {            
            if let Some(mask_bit) = generator.next() {
                let b = (address & (1 << (bit_no - 1))) >> (bit_no - 1);
                let b = b == 1;

                match mask_bit {
                    MaskBit::Zero => { // unchanged
                        address_accum.push(b);
                        _generate(address, bit_width, bit_no - 1, generator, address_accum)
                    },
                    MaskBit::One => { // push 1
                        address_accum.push(true);
                        _generate(address, bit_width, bit_no - 1, generator, address_accum)
                    },
                    MaskBit::DontCare => { // split and generate both
                        let first = {
                            let mut address_accum = address_accum.clone();
                            address_accum.push(true);

                            let generator = generator.clone();
                            _generate(address, bit_width, bit_no - 1, generator, &mut address_accum)
                        };

                        let second = {
                            // don't need to clone this time
                            address_accum.push(false); 
                            _generate(address, bit_width, bit_no - 1, generator, address_accum)
                        };

                        Box::new(first.chain(second))
                    }
                }
            } else {
                // finished 
                let result = address_accum.iter().fold(0 as usize, |a, v| {
                    (a << 1) | (if v { 1 } else { 0 })
                });
                Box::new(std::iter::once(result))
            }
        }

        let bit_width = self.bits.len();
        let mut accum = BitVec::with_capacity(bit_width);
        _generate(&address, bit_width, bit_width, self.bits.iter(), &mut accum)
    }
}

trait Maskable {
    fn mask(&self, mask: &Option<(u64, u64)>) -> u64;
}

impl Maskable for u64 {
    fn mask(&self, mask: &Option<(u64, u64)>) -> u64 {
        if let Some(mask) = mask {
            Mask::apply_mask(*self, mask)
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
            Instruction::Mask(Mask::parse(mask))
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
    let mut mask: Option<(u64, u64)> = None;

    for ins in program {
        match ins {
            Instruction::Mask(m) => { mask = Some(m.load_mask()); },
            Instruction::Write(address, value) => { 
                let entry = memory.entry(*address).or_insert(0);
                *entry = value.mask(&mask); 
            },
        }
    }

    memory.values().fold(0u64, |a, v| a + *v)
}

#[aoc(day14, part2)]
pub fn solve_part2(program: &[Instruction]) -> u64 {
    let mut memory = HashMap::<usize, u64>::new();
    let mut mask: Option<&Mask> = None;

    for ins in program {
        match ins {
            Instruction::Mask(m) => { mask = Some(&m); },
            Instruction::Write(address, value) => { 
                if let Some(mask) = mask {
                    for gen_address in mask.all_addresses_iter(address) {
                        let entry = memory.entry(gen_address).or_insert(0);
                        *entry = *value; 
                    }
                }
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
    fn solve_part2_given1() {
        let program = input_generator(
            "mask = 000000000000000000000000000000X1001X\n\
             mem[42] = 100\n\
             mask = 00000000000000000000000000000000X0XX\n\
             mem[26] = 1"
        );

        assert_eq!(solve_part2(&program), 208);
    }

    #[test]
    fn generate_addresses_given1() {
        let mask = Mask::parse("000000000000000000000000000000X1001X");
        let address = 42;

        // 000000000000000000000000000000011010  (decimal 26)
        // 000000000000000000000000000000011011  (decimal 27)
        // 000000000000000000000000000000111010  (decimal 58)
        // 000000000000000000000000000000111011  (decimal 59)
        assert_eq!(
            mask.all_addresses_iter(&address).collect::<Vec<_>>(),
            vec![59, 58, 27, 26],
        )
    }

    #[test]
    fn input_generator_given1() {
        let program = input_generator(GIVEN_INPUT1);

        assert_eq!(
            program,
            vec![
                Instruction::Mask(
                    Mask { 
                        bits: std::iter::repeat(MaskBit::DontCare).take(29)
                            .chain(std::iter::once(MaskBit::One))
                            .chain(std::iter::repeat(MaskBit::DontCare).take(4))
                            .chain(std::iter::once(MaskBit::Zero))
                            .chain(std::iter::once(MaskBit::DontCare))
                            .collect::<Vec<_>>(),
                    }
                ),
                Instruction::Write(8, 11),
                Instruction::Write(7, 101),
                Instruction::Write(8, 0),
            ]
        )
    }
}