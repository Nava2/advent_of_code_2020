use std::hash::Hash;
use std::cmp::PartialEq;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
pub enum Direction {
    Front,
    Back,
    Left,
    Right,
}

impl Direction {
    pub fn parse(d: char) -> Direction {
        match d {
            'F' => Direction::Front,
            'B' => Direction::Back,
            'L' => Direction::Left,
            'R' => Direction::Right,
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Vec<Direction>> {
    input.lines()
        .map(|line| line.chars().map(Direction::parse).collect())
        .collect()
}

const ROW_PARTITION_COUNT: usize = 7;

#[aoc(day5, part1)]
pub fn solve_part1(seat_descriptions: &[Vec<Direction>]) -> usize {
    seat_descriptions.iter()
        .map(|desc| { 
            
        })
        .count()
}
