use std::marker::Copy;
use std::cmp::PartialEq;
use std::cell::RefCell;

const TREE_CHAR: char = '#';

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Space {
    Tree,
    Free,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars().map(|c| match c {
                TREE_CHAR => Space::Tree,
                _ => Space::Free,
            })
            .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(slope: &[Vec<Space>]) -> usize {
    let x_bound = slope[0].len();

    let pattern = StepPattern { x_step: 3, y_step: 1 };
    let mut traversal = Traversal { 
        pattern,
        position: Position { x: 0, y: 0 },
        tree_count: 0,
    };

    while !traversal.is_complete(slope) {
        traversal.step(slope, x_bound);
    }

    traversal.tree_count
}

#[derive(Debug, Clone, Copy)]
struct StepPattern {
    x_step: usize,
    y_step: usize,
}

static STEP_PATTERNS: [&StepPattern; 5] = [
    &StepPattern { x_step: 1, y_step: 1 },
    &StepPattern { x_step: 3, y_step: 1 },
    &StepPattern { x_step: 5, y_step: 1 },
    &StepPattern { x_step: 7, y_step: 1 },
    &StepPattern { x_step: 1, y_step: 2 },
];

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(&mut self, step: &StepPattern, x_bound: usize) {
        self.x = (self.x + step.x_step) % x_bound;
        self.y += step.y_step;
    }
}

#[derive(Debug, Clone, Copy)]
struct Traversal {
    pattern: StepPattern,
    position: Position,
    tree_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum TraversalState {
    Running,
    Complete,
}

impl Traversal {
    fn is_complete(&self, slope: &[Vec<Space>]) -> bool {
        self.position.y >= slope.len()
    }

    fn step(&mut self, slope: &[Vec<Space>], x_bound: usize) -> TraversalState {
        self.position.step(&self.pattern, x_bound);

        if self.is_complete(slope) {
            return TraversalState::Complete
        }

        if let Space::Tree = slope[self.position.y][self.position.x] { 
            self.tree_count += 1 
        }

        if self.is_complete(slope) {
            TraversalState::Complete
        }
        else {
            TraversalState::Running
        }
    }
}

#[aoc(day3, part2)]
pub fn solve_part2(slope: &[Vec<Space>]) -> usize {
    let x_bound = slope[0].len();

    let traversals: Vec<RefCell<Traversal>> = STEP_PATTERNS.iter()
        .map(|&pattern| {
            Traversal {
                pattern: *pattern,
                position: Position { x: 0, y: 0 },
                tree_count: 0,
            }
        })
        .map(RefCell::new)
        .collect();

    let mut current_state: Vec<TraversalState> = vec![TraversalState::Running; traversals.len()];

    while current_state.iter().any(|t| *t == TraversalState::Running) {
        for (i, traversal) in traversals.iter().enumerate() {
            current_state[i] = traversal.borrow_mut().step(slope, x_bound);
        }
    }

    traversals.iter().map(|t| t.borrow().tree_count).product()
}