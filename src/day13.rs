#[derive(Debug, PartialEq)]
pub struct BusNotes {
    first_timestamp: u32,
    ids: Vec<Option<u32>>,
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
                .map(|s| {
                    if s == "x" {
                        None
                    }
                    else {
                        Some(s.parse::<u32>().unwrap())
                    }
                })
                .collect(),
    }
}

fn find_earliest_bus_time(bus_notes: &BusNotes) -> Option<(u32, u32)> {
    let bus_ids = bus_notes.ids.iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();
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

fn find_first_matching_time(seed_time: u64, bus_notes: &BusNotes) -> u64 {
    let bus_ids = &bus_notes.ids;

    let combinations_to_check = {
        let mut vec = bus_ids.iter()
            .enumerate()
            .flat_map(|(i, id)| id.map(|id| (i as u64, id as u64)))
            .collect::<Vec<_>>();

        vec.sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
        vec
    };
    println!("combinations_to_check = {:?}", combinations_to_check);

    let (window_offset, step) = combinations_to_check[0];
    let combinations_to_check = &combinations_to_check[1..];
    
    let mut time = step * (seed_time / step + 1);
    println!("window_offset = {}, step = {}, time = {}", window_offset, step, time);

    loop {
        let check_time = time - window_offset;
        let all_good = combinations_to_check.iter()
            .all(|(time_offset, id)| (check_time + time_offset) % id == 0);

        if all_good {
            break check_time;
        }

        time += step;
    }
}

#[aoc(day13, part2)]
pub fn solve_part2(bus_notes: &BusNotes) -> u64 {
    find_first_matching_time(100_000_000_000_000, &bus_notes)
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
                ids: vec![
                    Some(7),
                    Some(13),
                    None,
                    None,
                    Some(59),
                    None,
                    Some(31),
                    Some(19)
                ],
            }
        );
    }

    #[test]
    fn solve_part_given1() {
        let bus_notes = input_generator(GIVEN_INPUT_1);
        
        let earliest_time = find_earliest_bus_time(&bus_notes).unwrap();
        assert_eq!(earliest_time, (59,944));
    }

    #[test]
    fn solve_part2_given1() {
        let bus_notes = input_generator(GIVEN_INPUT_1);
        let valid_time = find_first_matching_time(0, &bus_notes);

        assert_eq!(valid_time, 1068781);
    }

    #[test]
    fn solve_part2_given2() {
        let bus_notes = input_generator(
            "0\n\
             17,x,13,19");
        let valid_time = find_first_matching_time(3400, &bus_notes);

        assert_eq!(valid_time, 3417);
    }

    #[test]
    fn solve_part2_given3() {
        let bus_notes = input_generator(
            "0\n\
             67,7,59,61");
        let valid_time = find_first_matching_time(750_000, &bus_notes);

        assert_eq!(valid_time, 754018);
    }

    #[test]
    fn solve_part2_given4() {
        let bus_notes = input_generator(
            "0\n\
             67,7,x,59,61");
        let valid_time = find_first_matching_time(0, &bus_notes);

        assert_eq!(valid_time, 1261476);
    }

    #[test]
    fn solve_part2_given5() {
        let bus_notes = input_generator("\n\
            0\n\
            1789,37,47,1889");
        let valid_time = find_first_matching_time(0, &bus_notes);

        assert_eq!(valid_time, 1202161486);
    }
}