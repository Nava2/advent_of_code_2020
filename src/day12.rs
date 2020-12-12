#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(_adaptors: &[u32]) -> usize {
    panic!()
}

#[cfg(test)]
mod tests {
    // use super::*;

    // const GIVEN_INPUT_1: [u32; 11] = [
    //     16,
    //     10,
    //     15,
    //     5,
    //     1,
    //     11,
    //     7,
    //     19,
    //     6,
    //     12,
    //     4,
    // ];
}