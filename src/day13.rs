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

fn find_earliest_bus_time(bus_notes: &BusNotes) -> Option<(u32, u32)> {
    let bus_ids = &bus_notes.ids;
    let first_timestamp = bus_notes.first_timestamp;

    let max_id = bus_ids.iter().max()?;
    (first_timestamp..(first_timestamp + max_id + 1)).filter_map(|time| {
            let id = bus_ids.iter().find(|id| time % **id == 0)?;
            Some((*id, time))
        })
        .next()
}

#[aoc(day13, part1)]
pub fn solve_part1(bus_notes: &BusNotes) -> u32 {
    let (id, time) = find_earliest_bus_time(&bus_notes).unwrap();
    let waiting_time = time - bus_notes.first_timestamp;
    waiting_time * id
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

    #[test]
    fn solve_part_given1() {
        let bus_notes = input_generator(GIVEN_INPUT_1);
        
        let earliest_time = find_earliest_bus_time(&bus_notes).unwrap();
        assert_eq!(earliest_time, (59, 944));
    }
}