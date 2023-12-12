pub fn part_one(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 11 part 1: {}", elapsed.as_nanos());
    ans as i128
}

fn part_one_inner(data: &str) -> i32 {
    let mut universe =
        data.lines()
            .enumerate()
            .fold(Universe::default(), |mut universe, (y, line)| {
                line.chars().enumerate().for_each(|(x, pnt)| {
                    let kind = match pnt {
                        '#' => Kind::Galaxy,
                        '.' => Kind::Space,
                        _ => unreachable!(),
                    };
                    let pnt = Point {
                        x: x as i32,
                        y: y as i32,
                        kind,
                    };
                    universe.add_pnt(pnt);
                });
                universe
            });
    universe.expand();

    println!(
        "Galaxies: {:?}",
        universe
            .points
            .iter()
            .filter(|p| p.kind == Kind::Galaxy)
            .collect::<Vec<_>>()
    );

    42
}

pub fn part_two(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);
    let elapsed = now.elapsed();
    println!("Day 11 part 2: {}", elapsed.as_nanos());
    ans as i128
}

fn part_two_inner(_data: &str) -> i32 {
    42
}

#[derive(Debug, Clone, Default)]
struct Universe {
    points: Vec<Point>,
    width: i32,  // 0 indexed
    height: i32, // 0 indexed
}

#[derive(Debug, PartialEq, Clone)]
enum Kind {
    Space,
    Galaxy,
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    kind: Kind,
}

impl Universe {
    fn add_pnt(&mut self, pnt: Point) {
        if pnt.x > self.width {
            self.width = pnt.x;
        }
        if pnt.y > self.height {
            self.height = pnt.y
        }
        self.points.push(pnt);
    }

    fn expand(&mut self) {
        let mut index = 0;
        while index <= self.width {
            let galaxies = self
                .points
                .iter()
                .filter(|p| p.kind == Kind::Galaxy && p.x == index)
                .count();
            if galaxies == 0 {
                println!("Cloning col at {index}");
                // Shift everything right once
                let mut cloned_points = vec![];
                self.points.iter_mut().for_each(|p| {
                    if p.x == index {
                        cloned_points.push(p.clone());
                    }
                    if p.x >= index {
                        p.x += 1;
                    }
                });
                self.width += 1;
                self.points.append(&mut cloned_points);
                index += 2;
            } else {
                index += 1;
            }
        }
        let mut index = 0;
        while index <= self.height {
            let galaxies = self
                .points
                .iter()
                .filter(|p| p.kind == Kind::Galaxy && p.y == index)
                .count();
            if galaxies == 0 {
                println!("Cloning row at {index}");
                // Shift everything right once
                let mut cloned_points = vec![];
                self.points.iter_mut().for_each(|p| {
                    if p.y == index {
                        cloned_points.push(p.clone());
                    }
                    if p.y >= index {
                        p.y += 1;
                    }
                });
                self.height += 1;
                self.points.append(&mut cloned_points);
                index += 2;
            } else {
                index += 1;
            }
        }
    }
}
