use std::hash::Hash;
use std::cmp::PartialEq;
use bit_vec::BitVec;
use std::ops::Range;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing() {
        assert_eq!(input_generator("abc")[0], vec![
            'a', 'b', 'c'
        ]);
    }
}
