#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect()
}

fn find_sum(preamble: &[usize], n: &usize) -> bool {
    for (i, left) in preamble.iter().enumerate().filter(|(_, l)| *l <= n) {
        for right in preamble.iter().skip(i).filter(|r| *r <= n) {
            if left + right == *n {
                return true;
            }
        }
    }

    false
}

fn find_weak_value(encoded: &[usize], preamble_n: usize) -> Option<usize> {
    let preamble_iter = (preamble_n..encoded.len())
        .map(|i| &encoded[i - preamble_n..i]);

    for (preamble, n) in preamble_iter.zip(encoded.iter().skip(preamble_n)) {
        if !find_sum(preamble, n) {
            return Some(*n);
        }       
    }

    None
}

#[aoc(day9, part1)]
pub fn solve_part1(encoded: &[usize]) -> usize {
    find_weak_value(encoded, 25).unwrap()
}

fn find_contiguous_weak_sum(weak_value: &usize, encoded: &[usize]) -> Option<(usize, usize)> {
    let valid_slices = encoded.iter()
        .take(encoded.len() - 1)
        .enumerate()
        .filter(|(_, v)| *v < weak_value)
        .map(|(i, _)| &encoded[i..]);

    for slice in valid_slices {
        let mut current_sum = 0;
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for p in slice.iter().cloned() {
            current_sum += p;
            min = std::cmp::min(min, p);
            max = std::cmp::max(max, p);

            if current_sum == *weak_value {
                return Some((min, max))
            }
        }
    }

    None
}

#[aoc(day9, part2)]
pub fn solve_part2(encoded: &[usize]) -> usize {
    let weak_value = find_weak_value(encoded, 25).unwrap();

    let (min, max) = find_contiguous_weak_sum(&weak_value, encoded).unwrap();
    min + max
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
        let result = find_weak_value(&input, preamble_n);
        assert_eq!(result, Some(127));
    }

    #[test]
    fn solve_given_input_part2() {
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

        let encoded = input_generator(input);
        let preamble_n = 5;
        let weak_value = find_weak_value(&encoded, preamble_n).unwrap();

        let min_max = find_contiguous_weak_sum(&weak_value, &encoded);
        assert_eq!(min_max, Some((15, 47)));        
    }
}