use regex::Regex;

#[derive(Debug, Clone)]
pub struct Policy {
    character: char,
    range: (usize, usize),
}

#[derive(Debug)]
pub struct Line {
    policy: Policy,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Line> {
    let line_re = Regex::new(r"^(\d+)-(\d+)\s+(\w):\s+(\w+)$").unwrap();

    input
        .lines()
        .map(|line| {
            let cap = &line_re.captures(line).unwrap();

            let lower_bound = cap[1].parse::<usize>().unwrap();
            let upper_bound = cap[2].parse::<usize>().unwrap();
            let character = cap[3].chars().next().unwrap();
            let password = String::from(&cap[4]);

            Line {
                policy: Policy {
                    character,
                    range: (lower_bound, upper_bound),
                },
                password,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|&line| {
            let policy = &line.policy;

            let char_count = line.password
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == policy.character)
                .count();

            let range = policy.range;
            char_count >= range.0 && char_count < (range.1 + 1)
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|&line| {
            let policy = &line.policy;

            let (first, second) = (policy.range.0 - 1, policy.range.1 - 1);
            let first_char = line.password.chars().nth(first).unwrap();
            let second_char = line.password.chars().nth(second).unwrap(); 
            (first_char == policy.character) ^ (second_char == policy.character)
        })
        .count()
}