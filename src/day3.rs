const TREE_CHAR: char = '#';

#[derive(Debug, Clone)]
pub enum Space {
    Tree,
    Free,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars().map(|c| match c {
                TREE_CHAR => Space::Tree,
                _ => Space::Free,
            })
            .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<Space>]) -> usize {
    todo!()
}