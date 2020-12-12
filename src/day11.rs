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

fn count_occupied_sight(layout: &[Vec<Space>], i: usize, j: usize) -> usize {
    // [i - 1, j - 1][i - 1, j][i - 1, j + 1]
    // [    i, j - 1][    i, j][    i, j + 1]
    // [i + 1, j - 1][i + 1, j][i + 1, j + 1]
    
    let mut count = 0;

    fn check_diag(layout: &[Vec<Space>], row_len: &usize, i: &usize, j: &usize, walk: (i32, i32)) -> bool {
        let mut i = *i as i64 + walk.0 as i64;
        let mut j = *j as i64 + walk.1 as i64;

        loop {
            if i < 0 || j < 0 {
                return false
            }

            let ui = i as usize;
            let uj = j as usize;

            if ui >= layout.len() || uj >= *row_len {
                return false
            }

            match &layout[i as usize][j as usize] {
                Space::Occupied => return true,
                Space::Empty => return false,
                _ => {}
            }

            i += walk.0 as i64;
            j += walk.1 as i64;
        }
    }

    let row_len = layout[0].len();

    for i_offset in -1..=1 {
        for j_offset in -1..=1 {
            if i_offset == 0 && j_offset == 0 {
                continue;
            }

            if check_diag(&layout, &row_len, &i, &j, (i_offset, j_offset)) {
                count += 1;
            }
        }
    }

    count
}

type CountOccupiedFn = dyn Fn(&[Vec<Space>], usize, usize) -> usize;

fn iterate_occupied(layout: &[Vec<Space>], count_occupied: &CountOccupiedFn, occupied_req: usize) -> (Vec<Vec<Space>>, bool){
    let mut mutated = false;
    let new_layout = layout.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, seat)| match seat {
                    Space::Occupied => {
                        if count_occupied(layout, i, j) >= occupied_req {
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

fn solve(layout: &[Vec<Space>], count_occupied: &CountOccupiedFn, occupied_req: usize) -> usize {
    let mut mutated = true;
    let mut new_layout = layout.to_vec();

    while mutated {
        let result = iterate_occupied(&new_layout, count_occupied, occupied_req);
        mutated = result.1;
        new_layout = result.0;
    }

    new_layout.into_iter()
        .flat_map(|r| r.into_iter().filter(|s| *s == Space::Occupied))
        .count()
}

#[aoc(day11, part1)]
pub fn solve_part1(layout: &[Vec<Space>]) -> usize {
    solve(layout, &count_occupied, 4)
}

#[aoc(day11, part2)]
pub fn solve_part2(layout: &[Vec<Space>]) -> usize {
    solve(&layout, &count_occupied_sight, 5)
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

    #[test]
    fn check_sight_count_given() {
        let layout = input_generator(
            ".......#.\n\
             ...#.....\n\
             .#.......\n\
             .........\n\
             ..#L....#\n\
             ....#....\n\
             .........\n\
             #........\n\
             ...#.....");

        assert_eq!(8, count_occupied_sight(&layout, 4, 3));
    }

    #[test]
    fn check_sight_count_given_2() {
        let layout = input_generator(
            ".............\n\
             .L.L.#.#.#.#.\n\
             .............");

        assert_eq!(0, count_occupied_sight(&layout, 1, 1));
        assert_eq!(1, count_occupied_sight(&layout, 1, 3));
    }

    #[test]
    fn check_sight_count_given_3() {
        let layout = input_generator(
            ".##.##.\n\
             #.#.#.#\n\
             ##...##\n\
             ...L...\n\
             ##...##\n\
             #.#.#.#\n\
             .##.##.");

        assert_eq!(0, count_occupied_sight(&layout, 3, 3));
    }

    #[test]
    fn solve_part2_given_1() {
        let layout = input_generator(
            "L.LL.LL.LL\n\
             LLLLLLL.LL\n\
             L.L.L..L..\n\
             LLLL.LL.LL\n\
             L.LL.LL.LL\n\
             L.LLLLL.LL\n\
             ..L.L.....\n\
             LLLLLLLLLL\n\
             L.LLLLLL.L\n\
             L.LLLLL.LL");

        assert_eq!(26, solve_part2(&layout));
    }
}