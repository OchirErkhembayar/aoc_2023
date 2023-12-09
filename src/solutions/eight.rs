use std::collections::HashMap;
use std::iter::Iterator;

const LEFT: char = 'L';
const RIGHT: char = 'R';

pub fn part_one(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 8 part 1: {}", elapsed.as_millis());
    ans
}

fn part_one_inner(data: &str) -> i128 {
    let mut lines = data.lines();
    let turns = {
        let mut tv = Vec::new();
        lines.next().unwrap().chars().for_each(|c| {
            tv.push(c);
        });
        Turns {
            turns: tv,
            index: 0,
        }
    };
    lines.next();

    let mut positions = HashMap::new();
    for line in lines {
        let (current, rl) = line.split_once(" = ").unwrap();
        let (left, right) = rl.split_once(", ").unwrap();
        let left = left[1..].to_owned();
        let right = right[..right.len() - 1].to_owned();
        let position = Position { left, right };
        positions.insert(current.to_string(), position);
    }
    let mut map = Map {
        current: "AAA".to_string(),
        positions,
        turns,
        target: "ZZZ".to_string(),
    };
    map.travel_part_one()
}

#[derive(Debug)]
struct Turns {
    turns: Vec<char>,
    index: usize,
}

impl Turns {
    fn next(&mut self) -> char {
        let turn = self.turns[self.index];
        if self.index == self.turns.len() - 1 {
            self.index = 0
        } else {
            self.index += 1;
        }
        turn
    }
}

#[derive(Debug)]
struct Map {
    current: String,
    positions: HashMap<String, Position>,
    turns: Turns,
    target: String,
}

#[derive(Debug)]
struct Position {
    left: String,
    right: String,
}

impl Map {
    fn _add_position(&mut self, element: String, left: String, right: String) {
        let position = Position { left, right };

        self.positions.insert(element, position);
    }

    fn travel_part_one(&mut self) -> i128 {
        let mut steps = 0;
        while self.current != self.target {
            let turn = self.turns.next();
            let position = self.positions.get(self.current.as_str()).unwrap();
            steps += 1;
            self.current = match turn {
                LEFT => position.left.to_owned(),
                RIGHT => position.right.to_owned(),
                _ => unreachable!(),
            };
        }
        steps
    }

    fn travel_part_two(&mut self) -> i128 {
        let mut elements = self
            .positions
            .iter()
            .filter_map(|(current, _)| {
                if current.ends_with('A') {
                    Some((current.to_owned(), 0))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let endings = self
            .positions
            .iter()
            .filter_map(|(current, _)| {
                if current.ends_with('Z') {
                    Some(current.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut steps = 0;
        while elements.iter().any(|(_, d)| *d == 0) {
            let turn = self.turns.next();
            steps += 1;
            for (current, d) in elements.iter_mut() {
                if *d != 0 {
                    continue;
                }
                let position = self.positions.get(current.as_str()).unwrap();
                *current = match turn {
                    LEFT => position.left.to_owned(),
                    RIGHT => position.right.to_owned(),
                    _ => unreachable!(),
                };
                if endings.contains(current) {
                    *d = steps;
                }
            }
        }
        elements.iter().map(|(_, d)| *d).fold(1, lcm)
    }
}

fn lcm(x: i128, y: i128) -> i128 {
    x * y / gcd(x, y)
}

fn gcd(mut x: i128, mut y: i128) -> i128 {
    while y != 0 {
        let tmp = x;
        x = y;
        y = tmp % y;
    }
    x
}

pub fn part_two(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);
    let elapsed = now.elapsed();
    println!("Day 8 part 2: {}", elapsed.as_millis());
    ans
}

fn part_two_inner(data: &str) -> i128 {
    let mut lines = data.lines();
    let turns = {
        let mut tv = Vec::new();
        lines.next().unwrap().chars().for_each(|c| {
            tv.push(c);
        });
        Turns {
            turns: tv,
            index: 0,
        }
    };
    lines.next();

    let mut positions = HashMap::new();
    for line in lines {
        let (current, rl) = line.split_once(" = ").unwrap();
        let (left, right) = rl.split_once(", ").unwrap();
        let left = left[1..].to_owned();
        let right = right[..right.len() - 1].to_owned();
        let position = Position { left, right };
        positions.insert(current.to_string(), position);
    }
    let mut map = Map {
        current: "AAA".to_string(),
        positions,
        turns,
        target: "ZZZ".to_string(),
    };
    map.travel_part_two()
}
