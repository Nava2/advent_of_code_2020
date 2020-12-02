#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            line.parse::<i32>().unwrap()
        })
        .collect()
}

const TARGET_VALUE: i32 = 2020;

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let possible_values: Vec<&i32> = input
        .iter()
        .filter(|&x| x <= &TARGET_VALUE)
        .collect();

    for i0 in 0..(possible_values.len() - 1) {
        for i1 in (i0+1)..possible_values.len() {
            let v0 = possible_values[i0];
            let v1 = possible_values[i1];
            if v0 + v1 == TARGET_VALUE {
                return v0 * v1;
            }
        }
    }

    unreachable!()
}

/*
#[cfg(test)]
mod tests {
    use super::{solve_part1 as part1};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    // ((( and (()(()( both result in floor 3.
    #[test]
    fn sample2() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    // ))((((( also results in floor 3.
    #[test]
    fn sample3() {
        assert_eq!(part1("))((((("), 3);
    }

    // ()) and ))( both result in floor -1 (the first basement level).
    #[test]
    fn sample4() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    // ))) and )())()) both result in floor -3.
    #[test]
    fn sample5() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample6() {
        assert_eq!(part2(")"), 1);
    }

    // ()()) causes him to enter the basement at character position 5.
    #[test]
    fn sample7() {
        assert_eq!(part2("()())"), 5);
    }
} */
