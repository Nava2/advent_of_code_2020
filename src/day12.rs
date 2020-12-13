#[derive(Clone, PartialEq, Debug)]
pub enum Action {
    North,
    South,
    East,
    West,
    Forward,
    Left,
    Right
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<(Action, u32)> {
    input.lines()
        .map(|line| {
            let trimmed = line.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();

            let action = match trimmed[0] {
                'N' => Action::North,
                'S' => Action::South,
                'E' => Action::East,
                'W' => Action::West,
                'F' => Action::Forward,
                'L' => Action::Left,
                'R' => Action::Right,
                c => unreachable!("Undefined behaviour = {}", c),
            };
            
            let value = trimmed.iter().skip(1).collect::<String>();
            let value = value.parse::<u32>().unwrap();
            (action, value)
        })
        .collect()
}

fn run_actions_part1(actions: &[(Action, u32)]) -> (i64, i64) {
    let mut y: i64 = 0;
    let mut x: i64 = 0;
    let mut d: f64 = 0.0;

    for (action, value) in actions.iter().map(|(a, v)| (a, *v as i64)) {
        match action {
            Action::North => { y += value; },
            Action::South => { y -= value; },
            Action::East => { x += value; },
            Action::West => { x -= value; },
            Action::Forward => {
                let d = d.to_radians();

                let x_off = (value as f64 * d.cos()) as i64;
                let y_off = (value as f64 * d.sin()) as i64;
                x += x_off;
                y += y_off;
            },
            Action::Left => { d += value as f64; },
            Action::Right => { d -= value as f64; },
        }
    }

    (x, y)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl std::ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, _rhs: &Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, _rhs: &Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Point;

    fn mul(self, _rhs: i64) -> Point {
        Point {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn rotate_around(&self, center: &Point, value_in_degrees: i32) -> Point {
        let s_minus_c = self - center;

        let rads = (value_in_degrees as f32).to_radians();
        let cos_rads = rads.cos().round() as i64;
        let sin_rads = rads.sin().round() as i64;

        let Point { x: smc_x, y: smc_y } = s_minus_c;

        Point { 
            x: center.x + (cos_rads * smc_x - sin_rads * smc_y), 
            y: center.y + (sin_rads * smc_x + cos_rads * smc_y), 
        }
    }
}

fn run_actions_part2(actions: &[(Action, u32)]) -> Vec<(Point, Point)> {
    let mut ship = Point::new(0, 0);
    let mut waypoint = Point::new(10, 1);

    std::iter::once((ship, waypoint)).chain(
        actions.iter()
            .map(|(a, v)| (a, *v as i64))
            .map(|(action, value)| {
                match action {
                    Action::North => {
                        waypoint.y += value; 
                    },
                    Action::South => {
                        waypoint.y -= value;
                    },
                    Action::East => {
                        waypoint.x += value;
                    },
                    Action::West => {
                        waypoint.x -= value;
                    },
                    Action::Forward => {
                        ship = ship + (waypoint * value);
                    },
                    Action::Left => { 
                        waypoint = waypoint.rotate_around(&Point { x: 0, y: 0 }, value as i32);
                    },
                    Action::Right => { 
                        waypoint = waypoint.rotate_around(&Point { x: 0, y: 0 }, -value as i32);
                    },
                };
        
                (ship, waypoint)
            })
        )
        .collect::<Vec<_>>()
}

#[aoc(day12, part1)]
pub fn solve_part1(actions: &[(Action, u32)]) -> usize {
    let (north, east) = run_actions_part1(actions);
    north.abs() as usize + east.abs() as usize
}

#[aoc(day12, part2)]
pub fn solve_part2(actions: &[(Action, u32)]) -> usize {
    let (Point { x: north, y: east }, _) = run_actions_part2(actions)
        .into_iter()
        .last()
        .unwrap();

    north.abs() as usize + east.abs() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn act_part2_given2() {
        let actions = input_generator(
            "L90\n\
             R90");
        let parts = run_actions_part2(&actions);

        assert_eq!(
            parts.into_iter()
                .map(|(_, p)| p)
                .collect::<Vec<_>>(),
            vec![
                Point::new(10, 1),
                Point::new(-1, 10),
                Point::new(10, 1),
            ]
        );
    }

    #[test]
    fn act_part2_given1() {
        // (0, 0) + (10, 1)
        // `F10` moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. 
        // The waypoint stays 10 units east and 1 unit north of the ship.
        // (100, 10) + (10, 1)
        //
        // N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
        // (100, 10) + (10, 4)
        //
        // F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38.
        // The waypoint stays 10 units east and 4 units north of the ship.
        // (170, 38) + (10, 4)
        //
        // R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship.
        // The ship remains at east 170, north 38.
        // (170, 38) + (4, -10)
        //
        // F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72.
        // The waypoint stays 4 units east and 10 units south of the ship.
        // (214, -72) + (4, -10)

        let actions = input_generator(
            "F10
             N3\n\
             F7\n\
             R90\n\
             F11");
        let parts = run_actions_part2(&actions);

        assert_eq!(
            parts,
            vec![
                (Point::new(0, 0), Point::new(10, 1)),
                (Point::new(100, 10), Point::new(10, 1)),
                (Point::new(100, 10), Point::new(10, 4)),
                (Point::new(170, 38), Point::new(10, 4)),
                (Point::new(170, 38), Point::new(4, -10)),
                (Point::new(214, -72), Point::new(4, -10)),
            ]
        );

        let (Point { x: east, y: north }, ..) = parts.iter()
            .cloned()
            .last()
            .unwrap();
        assert_eq!(-72, north);
        assert_eq!(214, east);
    }

    #[test]
    fn act_part1_given1() {
        let actions = input_generator(
            "F10
             N3\n\
             F7\n\
             R90\n\
             F11");
        let (north, east) = run_actions_part1(&actions);
        assert_eq!(17, north.abs());
        assert_eq!(8, east.abs());
    }

    #[test]
    fn parse_given1() {
        let actions = input_generator(
            "F10
             N3\n\
             F7\n\
             R90\n\
             F11");
        assert_eq!(
            actions, 
            vec![
                (Action::Forward, 10),
                (Action::North, 3),
                (Action::Forward, 7),
                (Action::Right, 90),
                (Action::Forward, 11),
            ]);
    }
}