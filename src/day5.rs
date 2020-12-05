use std::hash::Hash;
use std::cmp::PartialEq;
use bit_vec::BitVec;
use std::ops::Range;

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
const TOTAL_VALUES: usize = ROW_COUNT * COLUMN_COUNT;

pub fn calculate_seat_id(description: &[Direction]) -> usize {
    let row = description.iter()
        .take(ROW_PARTITION_COUNT)
        .fold(0..ROW_COUNT, |current_row_range, partition| {
            let partition_index = current_row_range.start + (current_row_range.end - current_row_range.start) / 2;
            match partition {
                Direction::Front => current_row_range.start..partition_index,
                Direction::Back => partition_index..current_row_range.end,
                _ => unreachable!(format!("Undefined partition={:?}", partition)),
            }
        }).start;
    let column = description.iter()
        .skip(ROW_PARTITION_COUNT)
        .fold(0..COLUMN_COUNT, |current_column_range, partition| {
            let partition_index = current_column_range.start + (current_column_range.end - current_column_range.start) / 2;
            match partition {
                Direction::Left => current_column_range.start..partition_index,
                Direction::Right => partition_index..current_column_range.end,
                _ => unreachable!(format!("Undefined partition={:?}", partition)),
            }
        }).start;
    
    // seat_id
    row * COLUMN_COUNT + column
}

fn id_at(row: usize, column: usize) -> usize {
    row * COLUMN_COUNT + column
}

fn id_to(id: &usize) -> (usize, usize) {
    (*id / COLUMN_COUNT, *id % COLUMN_COUNT)
}

#[aoc(day5, part1)]
pub fn solve_part1(seat_descriptions: &[Vec<Direction>]) -> usize {
    seat_descriptions.iter()
        .map(|desc| calculate_seat_id(&desc))
        .max()
        .unwrap()
}

fn load_id_map_with_only_valid_values(marked_ranges: &[Range<usize>]) -> BitVec {
    let mut invalid_markers = BitVec::with_capacity(TOTAL_VALUES);

    for i in 0..id_at(ROW_COUNT - 1, COLUMN_COUNT) {
        invalid_markers.push(marked_ranges.iter().any(|r| r.contains(&i)))
    }

    invalid_markers
}

fn print_table(markers: &BitVec) {
    if !cfg!(debug_assertions) {
        return;
    }

    for (id, v) in markers.iter().enumerate() {
        let (row, column) = id_to(&id);
        if column == 0 {
            if id > 0 {
                println!("|");
            }

            print!("R{:>03} |", row);
        }

        print!("{}", v as i32);
    }

    println!("!");
}

#[aoc(day5, part2)]
pub fn solve_part2(seat_descriptions: &[Vec<Direction>]) -> usize {
    let valid_values = id_at(1, 0)..id_at(ROW_COUNT - 1, COLUMN_COUNT);

    let given_ids = seat_descriptions.iter()
        .filter_map(|desc| {
            let id = calculate_seat_id(&desc);
            if valid_values.contains(&id) { Some(id) } else { None }
        });


    let invalid_ranges = vec![
        0..COLUMN_COUNT,
        id_at(ROW_COUNT - 1, 0)..id_at(ROW_COUNT - 1, COLUMN_COUNT)
    ];

    let mut invalid_markers = load_id_map_with_only_valid_values(&invalid_ranges);

    for given_id in given_ids {
        invalid_markers.set(given_id, true);
    }

    print_table(&invalid_markers);
    
    let result = invalid_markers.iter()
        .enumerate()
        .filter_map(|(i, v)| if !v { Some(i) } else { None })
        .find(|check_idx| {
            invalid_markers[check_idx - 1] && invalid_markers[check_idx + 1]
        }) // -1 / +1 are set
        .unwrap();

    print_table(&invalid_markers);

    if cfg!(debug_assertions) {
        let (row, column) = id_to(&result);
        println!("row={}, column={}, id={}", row, column, result);
    }

    result
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
