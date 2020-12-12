use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Space {
    Floor,
    Occupied,
    Empty,
}

impl Space {
    fn as_char(&self) -> char {
        match self {
            Space::Floor => '.',
            Space::Occupied => '#',
            Space::Empty => 'L',
        }
    }

    fn is_unoccupied(&self) -> bool {
        match self {
            Space::Floor | Space::Empty => true,
            Space::Occupied => false,
        }
    }
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Space>> {
    input.lines()
        .map(|line| line.chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                'L' => Space::Empty,
                '#' => Space::Occupied,
                '.' => Space::Floor,
                _ => unreachable!("Unsupported c = {}", c),
            }).collect::<Vec<_>>()
        )
        .collect()
}

fn count_occupied(layout: &[Vec<Space>], i: usize, j: usize) -> usize {
    // [i - 1, j - 1][i - 1, j][i - 1, j + 1]
    // [    i, j - 1][    i, j][    i, j + 1]
    // [i + 1, j - 1][i + 1, j][i + 1, j + 1]
    
    let mut count = 0;

    fn check_columns(row: &[Space], idx: usize) -> usize {
        let mut count = 0;
        if idx > 0 && !row[idx - 1].is_unoccupied() { 
            count += 1 
        } 

        if idx < row.len() - 1 && !row[idx + 1].is_unoccupied() { 
            count += 1;
        } 

        count
    }

    if i > 0 {
        let row = &layout[i - 1];
        count += check_columns(row, j);
        if !row[j].is_unoccupied() {
            count += 1;
        }
    }

    let row = &layout[i];
    count += check_columns(row, j);

    if i < layout.len() - 1 {
        let row = &layout[i + 1];
        count += check_columns(row, j);
        if !row[j].is_unoccupied() {
            count += 1;
        }
    }

    count
}

fn iterate_occupied(layout: &[Vec<Space>]) -> (Vec<Vec<Space>>, bool){
    let mut mutated = false;
    let new_layout = layout.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, seat)| match seat {
                    Space::Occupied => {
                        if count_occupied(layout, i, j) >= 4 {
                            mutated = true;
                            Space::Empty
                        }
                        else {
                            seat.clone()
                        }
                    },
                    Space::Empty => {
                        if count_occupied(layout, i, j) == 0 {
                            mutated = true;
                            Space::Occupied
                        }
                        else {
                            seat.clone()
                        }
                    },
                    Space::Floor => Space::Floor,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (new_layout, mutated)
}

#[aoc(day11, part1)]
pub fn solve_part1(layout: &[Vec<Space>]) -> usize {
    let mut mutated = true;
    let mut new_layout = layout.to_vec();
    print_layout(&new_layout);

    let mut iter_count = 0;
    while mutated {
        // println!("Iter={}", iter_count);
        let result = iterate_occupied(&new_layout);
        mutated = result.1;
        new_layout = result.0;

        print_layout(&new_layout);

        iter_count += 1;
        // println!();
    }

    return new_layout.into_iter()
        .flat_map(|r| r.into_iter().filter(|s| *s == Space::Occupied))
        .count();

    fn print_layout(layout: &[Vec<Space>]) {
        // for row in layout {
        //     println!("{}", row.iter().map(|s| format!("{}", s)).collect::<Vec<_>>().join(""));
        // }
        // println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT_1: &str = "L.LL.LL.LL\n\
        LLLLLLL.LL\n\
        L.L.L..L..\n\
        LLLL.LL.LL\n\
        L.LL.LL.LL\n\
        L.LLLLL.LL\n\
        ..L.L.....\n\
        LLLLLLLLLL\n\
        L.LLLLLL.L\n\
        L.LLLLL.LL";

    #[test]
    fn part1_provided_input() {
        let layout = input_generator(GIVEN_INPUT_1);

        let result = solve_part1(&layout);
        assert_eq!(37, result);
    }

    #[test]
    fn parse_input() {
        let layout = input_generator(GIVEN_INPUT_1);

        assert_eq!(
            layout[0],
            vec![
                Space::Empty, 
                Space::Floor, 
                Space::Empty, 
                Space::Empty, 
                Space::Floor, 
                Space::Empty, 
                Space::Empty, 
                Space::Floor, 
                Space::Empty, 
                Space::Empty, 
            ]
        );
    }
}