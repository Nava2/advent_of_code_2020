#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect()
}

fn find_sum_slow(preamble: &[usize], n: &usize) -> bool {
    let valid_numbers = preamble.iter()
            .filter(|&p| p < n)
            .cloned()
            .collect::<Vec<_>>();

    for left in valid_numbers.iter().take(valid_numbers.len() - 1) {
        for right in valid_numbers.iter().skip(1) {
            if left + right == *n {
                return true;
            }
        }
    }

    false
}

fn solve_part1_n(encoded: &[usize], preamble_n: usize) -> Option<usize> {
    let preamble_iter = (preamble_n..encoded.len())
        .map(|i| &encoded[i - preamble_n..i]);

    for (preamble, n) in preamble_iter.zip(encoded.iter().skip(preamble_n)) {
        if !find_sum_slow(preamble, n) {
            return Some(*n);
        }       
    }

    None
}

#[aoc(day9, part1)]
pub fn solve_part1(encoded: &[usize]) -> usize {
    solve_part1_n(encoded, 25).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_given_input_part1() {
        let input = "35\n\
                20\n\
                15\n\
                25\n\
                47\n\
                40\n\
                62\n\
                55\n\
                65\n\
                95\n\
                102\n\
                117\n\
                150\n\
                182\n\
                127\n\
                219\n\
                299\n\
                277\n\
                309\n\
                576";

        let input = input_generator(input);
        let preamble_n = 5;
        let result = solve_part1_n(&input, preamble_n);
        assert_eq!(result, Some(127));
    }
}