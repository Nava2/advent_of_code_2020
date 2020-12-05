use std::hash::Hash;
use std::cmp::PartialEq;

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
            _ => unreachable!(format!("Undefined input={:?}", d))
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Vec<Direction>> {
    input.lines()
        .map(|line| line.chars().map(Direction::parse).collect::<Vec<Direction>>())
        .collect::<Vec<Vec<Direction>>>()
}

const ROW_PARTITION_COUNT: usize = 7;
const ROW_COUNT: usize = 128;
const COLUMN_COUNT: usize = 8;

#[aoc(day5, part1)]
pub fn solve_part1(seat_descriptions: &[Vec<Direction>]) -> usize {
    seat_descriptions.iter()
        .map(|desc| { 
            let row = desc.iter()
                .take(ROW_PARTITION_COUNT)
                .fold(0..ROW_COUNT, |current_row_range, partition| {
                    let partition_index = current_row_range.start + (current_row_range.end - current_row_range.start) / 2;
                    match partition {
                        Direction::Front => current_row_range.start..partition_index,
                        Direction::Back => partition_index..current_row_range.end,
                        _ => unreachable!(format!("Undefined partition={:?}", partition)),
                    }
                }).start;
            let column = desc.iter()
                .skip(ROW_PARTITION_COUNT)
                .fold(0..COLUMN_COUNT, |current_column_range, partition| {
                    let partition_index = current_column_range.start + (current_column_range.end - current_column_range.start) / 2;
                    match partition {
                        Direction::Left => current_column_range.start..partition_index,
                        Direction::Right => partition_index..current_column_range.end,
                        _ => unreachable!(format!("Undefined partition={:?}", partition)),
                    }
                }).start;
            (row, column)
        })
        .map(|(row, column)| row * COLUMN_COUNT + column) // seat_id
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_part1(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_part1(&parsed)
    }

    #[test]
    fn input_parsing() {
        assert_eq!(input_generator("FBLR")[0], vec![
            Direction::Front,
            Direction::Back,
            Direction::Left,
            Direction::Right,
        ]);
        assert_eq!(input_generator("FBFBR")[0], vec![
            Direction::Front,
            Direction::Back,
            Direction::Front,
            Direction::Back,
            Direction::Right,
        ]);
    }

    #[test]
    fn solve_provided_part1() {
        assert_eq!(run_part1("FBFBBFFRLR"), 357);

        assert_eq!(run_part1("BFFFBBFRRR"), 567);
        assert_eq!(run_part1("FFFBBBFRRR"), 119);
        assert_eq!(run_part1("BBFFBBFRLL"), 820);
    }
}
