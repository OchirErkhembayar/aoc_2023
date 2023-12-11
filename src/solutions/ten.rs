pub fn part_one(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 10 part 1: {}", elapsed.as_micros());
    ans as i128
}

fn part_one_inner(data: &str) -> i32 {
    if true {
        return 0;
    }
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
    let mut tiles = vec![];
    for (y, line) in data.lines().enumerate() {
        line.chars().enumerate().for_each(|(x, c)| {
            tiles.push((x, y));
            if c != '.' {
                pipes.push(Pipe::new(x, y, c));
            }
        });
    }
    let mut maze = Maze::new(pipes);
    maze.traverse();
    let loop_pipes = maze.history;
    let tiles = tiles
        .into_iter()
        .filter(|&coords| !loop_pipes.iter().any(|p| p.position == coords))
        .collect::<Vec<_>>();
    let closed = tiles
        .iter()
        .filter(|&&coords| !encloses(coords, &loop_pipes))
        .collect::<Vec<_>>();
    let not_closed = tiles
        .iter()
        .filter(|&&coords| !encloses(coords, &loop_pipes))
        .collect::<Vec<_>>();

    closed.iter().fold(0, |acc, &coords| {
        if touches(*coords, &not_closed) {
            acc
        } else {
            println!("{:?}", coords);
            acc + 1
        }
    }) as i32
}

fn touches(coords: (usize, usize), dots: &Vec<&(usize, usize)>) -> bool {
    let (x, y) = coords;
    dots.iter()
        .filter(|d| {
            let (dx, dy) = d;
            ((*dx).abs_diff(x) == 1 && *dy == y) || ((*dy).abs_diff(y) == 1 && *dx == x)
        })
        .count()
        > 0
}

fn encloses(coords: (usize, usize), pipes: &Vec<Pipe>) -> bool {
    let (x, y) = coords;
    // find a coords with matching y and x smaller and larger (not hori)
    // and also matching x and y larger and smaller (not vert)
    // Also cannot be touching a coord that can escape
    let x_matches = pipes
        .iter()
        .filter(|p| p.position.1 == y && p.shape == Shape::Vert)
        .collect::<Vec<_>>();
    let y_matches = pipes
        .iter()
        .filter(|p| p.position.0 == x && p.shape == Shape::Hori)
        .collect::<Vec<_>>();
    let north_y = y_matches.iter().find(|p| p.position.1 < y).is_some();
    let east_x = x_matches.iter().find(|p| p.position.0 > x).is_some();
    let south_y = y_matches.iter().find(|p| p.position.1 > y).is_some();
    let west_x = x_matches.iter().find(|p| p.position.0 < x).is_some();
    if [(2, 6), (3, 6), (7, 6), (8, 6)].contains(&coords) {}
    let ans = south_y && north_y && east_x && west_x;
    if ans {
        /*
        println!("Coord: {:?}", coords);
        println!("X matches: {:?}", x_matches);
        println!("Y matches: {:?}", y_matches);
        println!("North found: {north_y}");
        println!("East found: {east_x}");
        println!("South found: {south_y}");
        println!("West : {west_x}");
        println!("Matching");
        // */
    }
    ans
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
        println!("Moves: {}", self.moves);
        (self.moves + 1) / 2
    }

    fn travel(&mut self) -> bool {
        let next_pipe = self.pipes.iter().enumerate().find(|(_, p)| {
            let (x, y) = p.position;
            match self.current.shape {
                Shape::Start => {
                    // Anything that is touching
                    let position = self.current.position;
                    (position.0.abs_diff(x) == 0 && position.1.abs_diff(y) == 1)
                        || (position.1.abs_diff(y) == 0 && position.0.abs_diff(x) == 1)
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
                            Direction::East => match p.shape {
                                Shape::Hori | Shape::SW | Shape::NW | Shape::Start => true,
                                _ => false,
                            },
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

#[derive(Debug, Clone)]
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
        if x_d.abs() > 1 {
            return None;
        }
        // other on top == -1 == north
        if y_d.abs() > 1 {
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
