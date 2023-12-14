pub fn part_one(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 11 part 1: {}", elapsed.as_nanos());
    ans as i128
}

fn part_one_inner(data: &str) -> i128 {
    run(data, 1)
}

pub fn part_two(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);
    let elapsed = now.elapsed();
    println!("Day 11 part 2: {}", elapsed.as_nanos());
    ans
}

fn part_two_inner(data: &str) -> i128 {
    run(data, 999999)
}

fn run(data: &str, expand_amount: i32) -> i128 {
    data.lines()
        .enumerate()
        .fold(Universe::default(), |mut universe, (y, line)| {
            line.chars().enumerate().for_each(|(x, pnt)| {
                if pnt == '#' {
                    let pnt = Galaxy {
                        x: x as i32,
                        y: y as i32,
                    };
                    universe.add_pnt(pnt);
                }
            });
            universe
        })
        .expand(expand_amount)
        .calc_edges()
}

#[derive(Debug, Clone, Default)]
struct Universe {
    points: Vec<Galaxy>,
    width: i32,  // 0 indexed
    height: i32, // 0 indexed
}

#[derive(Debug, Clone, PartialEq)]
struct Galaxy {
    x: i32,
    y: i32,
}

impl Universe {
    fn add_pnt(&mut self, pnt: Galaxy) {
        if pnt.x > self.width {
            self.width = pnt.x;
        }
        if pnt.y > self.height {
            self.height = pnt.y
        }
        self.points.push(pnt);
    }

    fn calc_edges(&self) -> i128 {
        self.points
            .iter()
            .map(|g| (g.x, g.y))
            .fold(0, |utotal, (x, y)| {
                self.points
                    .iter()
                    .filter(|p| {
                        if p.y > y {
                            true
                        } else if p.y == y {
                            p.x > x
                        } else {
                            false
                        }
                    })
                    .map(|g| (g.x, g.y))
                    .fold(utotal, |gtotal, (gx, gy)| {
                        let dx = x.abs_diff(gx) as i128;
                        let dy = y.abs_diff(gy) as i128;
                        dy + dx + gtotal
                    })
            })
    }

    fn expand(&mut self, amount: i32) -> &mut Self {
        let mut index = 0;
        while index <= self.width {
            let galaxies = self.points.iter().filter(|p| p.x == index).count();
            if galaxies == 0 {
                self.points.iter_mut().for_each(|p| {
                    if p.x > index {
                        p.x += amount;
                    }
                });
                self.width += amount;
                index += amount + 1;
            } else {
                index += 1;
            }
        }
        let mut index = 0;
        while index <= self.height {
            let galaxies = self.points.iter().filter(|p| p.y == index).count();
            if galaxies == 0 {
                self.points.iter_mut().for_each(|p| {
                    if p.y > index {
                        p.y += amount;
                    }
                });
                self.height += amount;
                index += amount + 1;
            } else {
                index += 1;
            }
        }
        self
    }
}
