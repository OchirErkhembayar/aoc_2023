use std::char;

pub fn part_one(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 12 part 1: {}", elapsed.as_micros());
    ans as i128
}

fn part_one_inner(data: &str) -> i32 {
    data.lines().map(|line| Row::from(line).calculate()).sum()
}

pub fn part_two(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);
    let elapsed = now.elapsed();
    println!("Day 12 part 1: {}", elapsed.as_micros());
    ans as i128
}

fn part_two_inner(data: &str) -> i32 {
    42
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Row {
    groups: Vec<Group>,
    group_nums: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Group {
    springs: Vec<Spring>,
}

impl Group {
    fn max_damages(&self) -> i32 {
        let length = self.springs.len() as i32;
        if length % 2 == 0 {
            if length == 1 {
                1
            } else {
                length / 2
            }
        } else {
            (length + 1) / 2
        }
    }

    // See how many different ways we can split this group up
    fn calculate(&self, grouping: i32) -> i32 {
        let len = self.springs.len() as i32;
        if len == grouping {
            return 1;
        }
        42
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '?' => Spring::Unknown,
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            _ => unreachable!(),
        }
    }
}

impl Row {
    fn calculate(&self) -> i32 {
        // Check the amount of chains the group can fit.

        42
    }
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        let (springs, group_nums) = s.split_once(' ').unwrap();
        let groups = springs
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| Group {
                springs: s.chars().map(|c| Spring::from(c)).collect::<Vec<_>>(),
            })
            .collect::<Vec<_>>();
        let group_nums = group_nums
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        Self { groups, group_nums }
    }
}
