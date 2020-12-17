use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

#[derive(Clone, Debug)]
struct PrevRun {
    earliest: i64,
    next: Option<i64>,
}

impl PrevRun {
    fn new(first: i64) -> PrevRun {
        PrevRun {
            earliest: first,
            next: None,
        }
    }

    fn next_spoken(&self) -> i64 {
        if let Some(next) = self.next {
            next - self.earliest
        }
        else {
            0
        }
    }

    fn shift_next(&mut self, next_next: i64) {
        if next_next == self.earliest {
            return // nothing to shift
        }

        if let Some(next) = self.next {
            self.earliest = next; // drop earliest
            self.next = Some(next_next);
        }
        else {
            self.next = Some(next_next);
        }
    }
}


fn part1_run(number_count: usize, turn_count: i64, starting_numbers: &[i64]) -> i64 {
    let mut last_turn_spoken_map = HashMap::<i64, PrevRun>::new();
    let mut last_spoken = 0;
    let mut turn = 0;

    for sn in starting_numbers.iter().map(|v| *v as i64) {
        last_turn_spoken_map.insert(sn, PrevRun::new(turn));
        // println!("T{:>3} -> {}", turn + 1, sn);

        turn += 1;
        last_spoken = sn;
    }

    // what is the 2020th _number_ spoken
    while last_turn_spoken_map.len() != number_count && turn < turn_count {
        let to_speak = if let Some(prev) = last_turn_spoken_map.get(&last_spoken) {
            // println!("T{:>3} -> prev={:?}", turn + 1, prev);

            // it was previously spoken
            prev.next_spoken()
        }
        else {
            0
        };

        // println!("T{:>3} -> last_spoken={}, speak={}", turn + 1, last_spoken, to_speak);

        last_turn_spoken_map.entry(to_speak)
            .and_modify(|e| (*e).shift_next(turn))
            .or_insert_with(|| PrevRun::new(turn));

        last_spoken = to_speak;
        turn += 1;
    }

    last_spoken
}

#[aoc(day15, part1)]
pub fn solve_part1(starting_numbers: &[i64]) -> i64 {
    part1_run(2020, 2020, starting_numbers)
}

#[aoc(day15, part2)]
pub fn solve_part2(starting_numbers: &[i64]) -> i64 {
    part1_run(usize::MAX, 30_000_000, starting_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_given_example() {
        assert_eq!(0, part1_run(2020, 10, &[0, 3, 6]));

        assert_eq!(436, part1_run(2020, 2020, &[0, 3, 6]));
    }

    #[test]
    fn p1_given() {
        assert_eq!(1, solve_part1(&[1, 3, 2]));
        assert_eq!(10, solve_part1(&[2, 1, 3]));
        assert_eq!(27, solve_part1(&[1, 2, 3]));
        assert_eq!(78, solve_part1(&[2, 3, 1]));
        assert_eq!(438, solve_part1(&[3, 2, 1]));
        assert_eq!(1836, solve_part1(&[3, 1, 2]));
    }

    #[test]
    fn p2_given_half1() {
        assert_eq!(175594, solve_part2(&[0, 3, 6]));
        // assert_eq!(2578, solve_part2(&[1, 3, 2]));
        // assert_eq!(3544142, solve_part2(&[2, 1, 3]));
        // assert_eq!(261214, solve_part2(&[1, 2, 3]));
    }

    #[test]
    fn p2_given_half2() {
        assert_eq!(6895259, solve_part2(&[2, 3, 1]));
        // assert_eq!(18, solve_part2(&[3, 2, 1]));
        // assert_eq!(362, solve_part2(&[3, 1, 2]));
    }
}