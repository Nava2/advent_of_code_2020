#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines()
        .map(|_line| {
            30
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(_actions: &[u32]) -> usize {
    panic!();
}

#[cfg(test)]
mod tests {
    // use super::*;
}