#[derive(Clone, PartialEq, Debug)]
pub enum Action {
    North,
    South,
    East,
    West,
    Forward,
    Left,
    Right
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<(Action, u32)> {
    input.lines()
        .map(|line| {
            let trimmed = line.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();

            let action = match trimmed[0] {
                'N' => Action::North,
                'S' => Action::South,
                'E' => Action::East,
                'W' => Action::West,
                'F' => Action::Forward,
                'L' => Action::Left,
                'R' => Action::Right,
                c => unreachable!("Undefined behaviour = {}", c),
            };
            
            let value = trimmed.iter().skip(1).collect::<String>();
            let value = value.parse::<u32>().unwrap();
            (action, value)
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(actions: &[(Action, u32)]) -> usize {
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_given1() {
        let actions = input_generator(
            "F10
             N3\n\
             F7\n\
             R90\n\
             F11");
        assert_eq!(
            actions, 
            vec![
                (Action::Forward, 10),
                (Action::North, 3),
                (Action::Forward, 7),
                (Action::Right, 90),
                (Action::Forward, 11),
            ]);
    }
}