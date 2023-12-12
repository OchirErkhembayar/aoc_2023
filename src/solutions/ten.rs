pub fn part_one(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 10 part 1: {}", elapsed.as_micros());
    ans as i128
}

fn part_one_inner(data: &str) -> i32 {
    let allpipes = data
        .lines()
        .enumerate()
        .fold(vec![], |mut pipes, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c != '.' {
                    pipes.push(Pipe::new(x, y, c));
                }
            });
            pipes
        });
    let mut maze = Maze::new(allpipes);
    maze.traverse();
    (maze.history.len() / 2) as i32
}

pub fn part_two(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);
    let elapsed = now.elapsed();
    println!("Day 10 part 2: {}", elapsed.as_micros());
    ans as i128
}

fn part_two_inner(data: &str) -> i32 {
    let mut pipes = vec![];
    for (y, line) in data.lines().enumerate() {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                pipes.push(Pipe::new(x, y, c));
            }
        });
    }
    let mut maze = Maze::new(pipes);
    maze.traverse();
    let loop_pipes = maze.history.iter().map(|p| p.position).collect::<Vec<_>>();
    let mut area = loop_pipes.windows(2).fold(0, |acc, positions| {
        let (x1, y1) = positions[0];
        let (x2, y2) = positions[1];
        acc + (x1 as i32 * y2 as i32) - (y1 as i32 * x2 as i32)
    });
    let (fx, fy) = loop_pipes.first().unwrap();
    let (lx, ly) = loop_pipes.last().unwrap();
    area += (*lx as i32) * (*fy as i32) - (*ly as i32) * (*fx as i32);
    area /= 2;
    area - (loop_pipes.len() as i32) / 2 + 1
}

#[derive(Debug, PartialEq, Clone)]
enum Shape {
    Vert,
    Hori,
    NW,
    SW,
    SE,
    NE,
    Start,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
struct Maze {
    pipes: Vec<Pipe>,
    moves: usize,
    current: Pipe,
    history: Vec<Pipe>,
}

impl Maze {
    fn new(mut pipes: Vec<Pipe>) -> Self {
        let pos = pipes.iter().position(|p| p.shape == Shape::Start).unwrap();
        let current = pipes[pos].clone();
        pipes.remove(pos);
        let history = vec![current.clone()];
        Self {
            pipes,
            moves: 0,
            current,
            history,
        }
    }

    fn traverse(&mut self) -> usize {
        self.travel();
        while self.travel() {}
        (self.moves + 1) / 2
    }

    fn travel(&mut self) -> bool {
        let next_pipe = self.pipes.iter().enumerate().find(|(_, p)| {
            match self.current.shape {
                Shape::Start => {
                    let (x, y) = self.current.position;
                    assert!(p.shape != Shape::Start);
                    let (nx, ny) = p.position;
                    if nx == x && y > 0 && y - 1 == ny {
                        return matches!(p.shape, Shape::SE | Shape::Vert | Shape::SW);
                    } else if nx == x && y + 1 == ny {
                        return matches!(p.shape, Shape::NE | Shape::Vert | Shape::NW);
                    } else if ny == y && x + 1 == nx {
                        return matches!(p.shape, Shape::NW | Shape::SW | Shape::Hori);
                    } else if ny == y && x > 0 && x - 1 == nx {
                        return matches!(p.shape, Shape::NE | Shape::SE | Shape::Hori);
                    }
                    false
                }
                Shape::Vert => {
                    // Top or down
                    if let Some(direction) = self.current.direction(p) {
                        match direction {
                            Direction::North => matches!(
                                p.shape,
                                Shape::Vert | Shape::SW | Shape::SE | Shape::Start
                            ),
                            Direction::South => matches!(
                                p.shape,
                                Shape::Vert | Shape::NW | Shape::NE | Shape::Start
                            ),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                Shape::Hori => {
                    if let Some(direction) = self.current.direction(p) {
                        match direction {
                            Direction::East => matches!(
                                p.shape,
                                Shape::Hori | Shape::SW | Shape::NW | Shape::Start
                            ),
                            Direction::West => matches!(
                                p.shape,
                                Shape::Hori | Shape::SE | Shape::NE | Shape::Start
                            ),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                Shape::NW => {
                    if let Some(direction) = self.current.direction(p) {
                        match direction {
                            Direction::North => matches!(
                                p.shape,
                                Shape::Vert | Shape::SW | Shape::SE | Shape::Start
                            ),
                            Direction::West => matches!(
                                p.shape,
                                Shape::Hori | Shape::SE | Shape::NE | Shape::Start
                            ),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                Shape::SW => {
                    if let Some(direction) = self.current.direction(p) {
                        match direction {
                            Direction::South => matches!(
                                p.shape,
                                Shape::Vert | Shape::NW | Shape::NE | Shape::Start
                            ),
                            Direction::West => matches!(
                                p.shape,
                                Shape::Hori | Shape::SE | Shape::NE | Shape::Start
                            ),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                Shape::SE => {
                    if let Some(direction) = self.current.direction(p) {
                        match direction {
                            Direction::South => matches!(
                                p.shape,
                                Shape::Vert | Shape::NW | Shape::NE | Shape::Start
                            ),
                            Direction::East => matches!(
                                p.shape,
                                Shape::Hori | Shape::SW | Shape::NW | Shape::Start
                            ),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                Shape::NE => {
                    if let Some(direction) = self.current.direction(p) {
                        match direction {
                            Direction::North => matches!(
                                p.shape,
                                Shape::Vert | Shape::SW | Shape::SE | Shape::Start
                            ),
                            Direction::East => matches!(
                                p.shape,
                                Shape::Hori | Shape::SW | Shape::NW | Shape::Start
                            ),
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
            }
        });
        if let Some((i, pipe)) = next_pipe {
            self.current = pipe.clone();
            self.pipes.remove(i);
            self.moves += 1;
            self.history.push(self.current.clone());
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Pipe {
    shape: Shape,
    position: (usize, usize),
}

impl Pipe {
    fn new(x: usize, y: usize, shape: char) -> Self {
        let shape = match shape {
            '|' => Shape::Vert,
            '-' => Shape::Hori,
            'L' => Shape::NE,
            'F' => Shape::SE,
            '7' => Shape::SW,
            'J' => Shape::NW,
            'S' => Shape::Start,
            _ => unreachable!(),
        };
        Self {
            shape,
            position: (x, y),
        }
    }

    fn direction(&self, other: &Self) -> Option<Direction> {
        let x_d = other.position.0 as i32 - self.position.0 as i32;
        let y_d = other.position.1 as i32 - self.position.1 as i32;
        if x_d.abs() > 1 || y_d.abs() > 1 {
            return None;
        }
        match x_d {
            0 => match y_d {
                1 => Some(Direction::South),
                -1 => Some(Direction::North),
                0 => None,
                _ => unreachable!(),
            },
            1 => {
                if y_d == 0 {
                    Some(Direction::East)
                } else {
                    None
                }
            }
            -1 => {
                if y_d == 0 {
                    Some(Direction::West)
                } else {
                    None
                }
            }
            _ => unreachable!(),
        }
    }
}
