use std::collections::HashMap;
use std::iter;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect()
}

fn sort_adaptors(adaptors: &[u32]) -> Vec<u32> {
    let mut adaptors = adaptors.to_vec();
    adaptors.sort_unstable();

    adaptors
}

fn compute_jolt_diffs(adaptors: &[u32]) -> (usize, usize) {
    let adaptors = sort_adaptors(adaptors);

    let mut one_diffs = 0;
    let mut three_diffs = 0;

    let left_iter = iter::once(&0).chain(adaptors.iter());
    let console_jolts = adaptors[adaptors.len() - 1] + 3;
    let right_iter = adaptors.iter().chain(iter::once(&console_jolts));

    for (left, right) in left_iter.zip(right_iter) {
        let diff = right - left;
        match diff {
            1 => one_diffs += 1,
            3 => three_diffs += 1,
            _ => {},
        };
    }

    (one_diffs, three_diffs)
}

fn compute_traversal_graph(adaptors: &[u32], console_jolts: &u32) -> HashMap<u32, Vec<u32>> {
    // pad with end points
    let adaptors = iter::once(0)
        .chain(adaptors.iter().cloned())
        .chain(iter::once(*console_jolts))
        .collect::<Vec<_>>();

    let mut result = HashMap::<u32, Vec<u32>>::with_capacity(adaptors.len());
    
    for (i, current) in adaptors.iter().take(adaptors.len() - 1).enumerate() {
        let options = adaptors[i+1..].iter()
            .take(3)
            .filter(|n| *n - current <= 3)
            .cloned()
            .collect::<Vec<_>>();        

        result.insert(*current, options);
    }

    result
}

fn count_possibilities(graph: &HashMap<u32, Vec<u32>>, terminal: &u32) -> usize { 
    let mut memory = HashMap::with_capacity(graph.len());

    fn count_possibilities_(graph: &HashMap<u32, Vec<u32>>, memory: &mut HashMap<u32, usize>, terminal: &u32, current: &u32) -> usize {
        if *current == *terminal {
            return 1;
        }

        let possibilities = &graph[current];
        possibilities.iter()
            .map(|p| {
                if *p == *terminal {
                    1
                } 
                else if let Some(memoized) = memory.get(p) {
                    *memoized
                }
                else {
                    let v = count_possibilities_(&graph, memory, terminal, p); 
                    memory.insert(*p, v); // memoize to avoid extra traversals
                    v
                }
            }) 
            .sum()
    }

    count_possibilities_(&graph, &mut memory, &terminal, &0)
}

#[aoc(day10, part1)]
pub fn solve_part1(adaptors: &[u32]) -> usize {
    let (one_diffs, three_diffs) = compute_jolt_diffs(&adaptors);
    one_diffs * three_diffs
}

#[aoc(day10, part2)]
pub fn solve_part2(adaptors: &[u32]) -> usize {
    let adaptors = sort_adaptors(adaptors);
    let console_jolts = adaptors[adaptors.len() - 1] + 3;

    let graph = compute_traversal_graph(&adaptors, &console_jolts);
    count_possibilities(&graph, &console_jolts)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT_1: [u32; 11] = [
        16,
        10,
        15,
        5,
        1,
        11,
        7,
        19,
        6,
        12,
        4,
    ];

    const GIVEN_INPUT_2: [u32; 31] = [ 
        28,
        33,
        18,
        42,
        31,
        14,
        46,
        20,
        48,
        47,
        24,
        23,
        49,
        45,
        19,
        38,
        39,
        11,
        1,
        32,
        25,
        35,
        8,
        17,
        7,
        9,
        4,
        2,
        34,
        10,
        3,
    ];

    #[test]
    fn given_input1_part2() {
        let input = GIVEN_INPUT_1;

        let adaptors = sort_adaptors(&input);
        let console_jolts = adaptors[adaptors.len() - 1] + 3;
    
        let graph = compute_traversal_graph(&adaptors, &console_jolts);
        println!("graph = {:?}", graph);
        assert_eq!(graph.len(), input.len() + 1);

        let count = count_possibilities(&graph, &console_jolts);
        assert_eq!(count, 8);
    }

    #[test]
    fn given_input2_part2() {
        let input = GIVEN_INPUT_2;

        let adaptors = sort_adaptors(&input);
        let console_jolts = adaptors[adaptors.len() - 1] + 3;
    
        let graph = compute_traversal_graph(&adaptors, &console_jolts);
        println!("graph = {:?}", graph);
        assert_eq!(graph.len(), input.len() + 1);

        let count = count_possibilities(&graph, &console_jolts);
        assert_eq!(count, 19208);
    }

    #[test]
    fn given_input1_part1() {
        let input = GIVEN_INPUT_1;

        let (one_diffs, three_diffs) = compute_jolt_diffs(&input);

        assert_eq!(7, one_diffs);
        assert_eq!(5, three_diffs);
    }

    #[test]
    fn given_input2_part1() {
        let input = GIVEN_INPUT_2;

        let (one_diffs, three_diffs) = compute_jolt_diffs(&input);

        assert_eq!(22, one_diffs);
        assert_eq!(10, three_diffs);
    }
}