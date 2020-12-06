use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut result = Vec::<Vec<Vec<char>>>::new();

    let mut current_group = Vec::<Vec<char>>::new();
    for line in input.lines() {
        if line.chars().count() == 0 {
            result.push(current_group);
            current_group = Vec::new();
            continue;
        }

        current_group.push(line.chars().collect());
    }

    result.push(current_group);
    result
}

fn collect_set(input: &[char]) -> HashSet<char> {
    input.iter().cloned().collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(groups: &[Vec<Vec<char>>]) -> usize {
    groups.iter()
        .map(|g| g.iter().cloned().flatten().collect::<HashSet<char>>())
        .map(|common_answers| common_answers.len())
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(groups: &[Vec<Vec<char>>]) -> usize {
    let all_chars_set = ('a'..'z').collect::<HashSet<char>>();
    groups.iter()
        .map(|group| {
            let group = group.iter()
                .map(|v| collect_set(v))
                .collect::<Vec<HashSet<char>>>();

            // seed with first
            group
                .iter()
                .skip(1)
                .fold(group[0].clone(), |group_set, person| {
                    group_set.intersection(&person)
                        .cloned()
                        .collect::<HashSet<char>>()
                })
        })
        .map(|shared| shared.len())
        .sum()
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
