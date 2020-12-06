use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut result = Vec::<Vec<Vec<char>>>::new();

    let mut current_group = Vec::<Vec<char>>::new();
    for line in input.lines() {
        if line.chars().count() == 0 {
            result.push(current_group);
            current_group = Vec::new();
        }

        current_group.push(line.chars().collect());
    }

    result.push(current_group);
    result
}

#[aoc(day6, part1)]
pub fn solve_part1(groups: &[Vec<Vec<char>>]) -> usize {
    groups.iter()
        .map(|g| g.iter().cloned().flatten().collect::<HashSet<char>>().len())
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(groups: &[Vec<Vec<char>>]) -> usize {
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing() {
        assert_eq!(
            input_generator("abc")[0], vec![
                'a', 'b', 'c'
            ].into_iter().collect::<HashSet<char>>());
        assert_eq!(
            input_generator("ac\nb\n\n")[0], vec![
                'a', 'b', 'c'
            ].into_iter().collect::<HashSet<char>>());
    }
}
