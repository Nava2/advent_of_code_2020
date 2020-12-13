#[derive(Debug, PartialEq)]
pub struct BusNotes {
    first_timestamp: u32,
    ids: Vec<u32>,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> BusNotes {
    let lines = input.lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
    
    BusNotes {
        first_timestamp: lines[0].parse::<u32>().unwrap(),
        ids: lines[1].split(',')
                .filter(|s| *s != "x")
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(_notes: &BusNotes) -> usize {
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT_1: &str = "\n\
        939\n\
        7,13,x,x,59,x,31,19";

    #[test]
    fn parse_input_given1() {
        let bus_notes = input_generator(GIVEN_INPUT_1);
        assert_eq!(
            bus_notes,
            BusNotes {
                first_timestamp: 939,
                ids: vec![7, 13, 59, 31, 19],
            }
        );
    }
}