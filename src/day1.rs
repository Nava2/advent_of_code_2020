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

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let possible_values: Vec<&i32> = input
        .iter()
        .filter(|&x| x <= &TARGET_VALUE)
        .collect();
    
    let v_len = possible_values.len();

    let memoized: Vec<Vec<i32>> = (0..(v_len - 2))
        .into_iter()
        .map(|i| {            
            let current = *possible_values[i];
            possible_values.iter()
                .take(v_len - 1)
                .skip(i + 1)
                .map(|v1| *v1 + current)
                .collect()
        })
        .collect();

    for i0 in 0..(v_len - 2) {
        for i1 in (i0+1)..(v_len - 1) {
            for i2 in (i1+1)..v_len {
                let v_zero_one = memoized[i0][i1 - i0 - 1];
                let v2 = possible_values[i2];
                if v_zero_one + v2 == TARGET_VALUE {
                    return possible_values[i0] * possible_values[i1]  * v2;
                }
            }
        }
    }

    unreachable!()
}