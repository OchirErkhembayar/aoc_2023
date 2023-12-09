pub fn part_one(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 9 part 1: {}", elapsed.as_micros());
    ans
}

fn part_one_inner(data: &str) -> i32 {
    let mut sum = 0;
    for line in data.lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let seq = Seq::new(nums);
        sum += seq.calc_next();
    }
    sum
}

pub fn part_two(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);
    let elapsed = now.elapsed();
    println!("Day 9 part 2: {}", elapsed.as_micros());
    ans
}

fn part_two_inner(data: &str) -> i32 {
    let mut sum = 0;
    for line in data.lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let seq = Seq::new(nums);
        sum += seq.calc_prev();
    }
    sum
}

#[derive(Debug)]
struct Seq {
    vals: Vec<i32>,
    child: Option<Box<Seq>>,
}

impl Seq {
    fn new(vals: Vec<i32>) -> Self {
        let child = if vals.iter().all(|&v| v == 0) {
            None
        } else {
            let mut diffs = vec![];
            for pair in vals.windows(2) {
                diffs.push(pair[1] - pair[0]);
            }
            Some(Box::new(Self::new(diffs)))
        };
        Seq { vals, child }
    }

    fn calc_next(&self) -> i32 {
        if let Some(seq) = &self.child {
            *self.vals.last().unwrap() + seq.calc_next()
        } else {
            0
        }
    }

    fn calc_prev(&self) -> i32 {
        let prev = if let Some(seq) = &self.child {
            *self.vals.first().unwrap() - seq.calc_prev()
        } else {
            0
        };
        prev
    }
}
