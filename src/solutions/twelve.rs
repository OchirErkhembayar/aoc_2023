use std::{
    char,
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub fn part_one(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 12 part 1: {}", elapsed.as_micros());
    println!("Part 1 ans: {ans}");
    ans as i128
}

fn part_one_inner(data: &str) -> i64 {
    let rows = data.lines().map(Row::from).collect::<Vec<_>>();

    let sum = Arc::new(Mutex::new((0, 1)));
    std::thread::scope(|s| {
        for row in rows {
            let sum = Arc::clone(&sum);
            s.spawn(move || {
                let comb = row.calculate();
                let mut s = sum.lock().unwrap();
                (*s).0 += comb;
                (*s).1 += 1;
                println!("s: {}", (*s).1);
                drop(s);
            });
        }
    });
    let s = sum.lock().unwrap();
    (*s).0
}

pub fn part_two(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_two_smarter(data);
    let elapsed = now.elapsed();
    println!("Day 12 part 2: {}", elapsed.as_secs());
    ans as i128
}

// This doesn't work.
fn _part_two_inner(data: &str) -> i64 {
    let rows = data.lines().map(Row::unfolded).collect::<Vec<_>>();

    let sum = Arc::new(Mutex::new((0, 1)));
    std::thread::scope(|s| {
        for row in rows {
            let sum = Arc::clone(&sum);
            s.spawn(move || {
                let comb = row.calculate();
                let mut s = sum.lock().unwrap();
                (*s).0 += comb;
                (*s).1 += 1;
                println!("s: {}", (*s).1);
                drop(s);
            });
        }
    });
    let s = sum.lock().unwrap();
    (*s).0
}

fn part_two_smarter(data: &str) -> i64 {
    data.lines()
        .map(|l| {
            let (springs, group_nums) = l.split_once(' ').unwrap();
            let springs = (0..5)
                .fold(String::new(), |mut acc, _| {
                    if !acc.is_empty() {
                        acc.push('?');
                    }
                    acc.push_str(springs);
                    acc
                })
                .chars()
                .map(Spring::from)
                .collect::<Vec<_>>();
            let group_nums = group_nums
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);
            let mut cache: HashMap<(usize, &[usize], usize), usize> = HashMap::new();
            do_it(&mut cache, springs.as_slice(), group_nums.as_slice(), 0) as i64
        })
        .sum()
}

// https://github.com/coravacav/AdventOfCode/blob/main/2023-12/rust/src/part2.rs
fn do_it<'a>(
    cache: &mut HashMap<(usize, &'a [usize], usize), usize>,
    springs: &'a [Spring],
    mut groups: &'a [usize],
    mut dmged: usize,
) -> usize {
    if cache.contains_key(&(springs.len(), groups, dmged)) {
        return *cache.get(&(springs.len(), groups, dmged)).unwrap();
    }

    if groups.is_empty() {
        return 0;
    }

    if Some(&dmged) == groups.first() {
        groups = &groups[1..];
        dmged = usize::MAX;

        if groups.is_empty() {
            return if springs.contains(&Spring::Damaged) {
                0
            } else {
                1
            };
        }
    }

    let Some(spring) = springs.first() else {
        return 0;
    };

    fn do_dmged<'a>(
        cache: &mut HashMap<(usize, &'a [usize], usize), usize>,
        springs: &'a [Spring],
        groups: &'a [usize],
        dmged: usize,
    ) -> usize {
        if dmged == usize::MAX {
            0
        } else {
            let res = do_it(cache, &springs[1..], groups, dmged + 1);
            cache.insert((springs.len() - 1, groups, dmged + 1), res);
            res
        }
    }

    fn do_operational<'a>(
        cache: &mut HashMap<(usize, &'a [usize], usize), usize>,
        springs: &'a [Spring],
        groups: &'a [usize],
        dmged: usize,
    ) -> usize {
        if dmged > 0 && dmged != usize::MAX {
            0
        } else {
            let res = do_it(cache, &springs[1..], groups, 0);
            cache.insert((springs.len() - 1, groups, 0), res);
            res
        }
    }

    match spring {
        Spring::Operational => do_operational(cache, springs, groups, dmged),
        Spring::Damaged => do_dmged(cache, springs, groups, dmged),
        Spring::Unknown => {
            do_operational(cache, springs, groups, dmged) + do_dmged(cache, springs, groups, dmged)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Row {
    groups: Vec<Group>,
    group_nums: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Group {
    springs: Vec<Spring>,
    required_indexes: HashMap<usize, Spring>,
    length: usize,
}

impl Group {
    fn calculate(&self, groupings: &[usize]) -> i64 {
        let mut combinations = 0;
        self.combinations(groupings, vec![], &mut combinations);
        println!("Calced: {combinations}");
        combinations
    }

    fn combinations(
        &self,
        groupings: &[usize],
        mut current_combination: Vec<Spring>,
        combos: &mut i64,
    ) {
        if groupings.is_empty() {
            let mut index = current_combination.len() - 1;
            while index + 1 < self.length {
                current_combination.push(Spring::Operational);
                index += 1;
                if self
                    .required_indexes
                    .get(&index)
                    .is_some_and(|s| *s != Spring::Operational)
                {
                    return;
                }
            }
            return;
        }

        let min_width: usize = groupings.iter().sum::<usize>() + groupings.len() - 1;
        let max_len = self.length - current_combination.len() - min_width;

        for i in 0..=max_len {
            let mut temp_comb = current_combination.clone();
            let index = temp_comb.len();
            for idx in index..index + i {
                temp_comb.push(Spring::Operational);
                if self
                    .required_indexes
                    .get(&idx)
                    .is_some_and(|s| *s != Spring::Operational)
                {
                    return;
                }
            }
            let index = temp_comb.len();
            let mut mismatch = false;
            for idx in index..index + groupings[0] {
                temp_comb.push(Spring::Damaged);
                if self
                    .required_indexes
                    .get(&idx)
                    .is_some_and(|s| *s != Spring::Damaged)
                {
                    mismatch = true;
                    break;
                }
            }
            if mismatch {
                continue;
            }
            if temp_comb.len() < self.length {
                temp_comb.push(Spring::Operational);
                if self
                    .required_indexes
                    .get(&(temp_comb.len() - 1))
                    .is_some_and(|s| *s != Spring::Operational)
                {
                    continue;
                }
            }
            self.combinations(&groupings[1..], temp_comb, combos);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
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
    fn calculate(self) -> i64 {
        self.groups[0].calculate(self.group_nums.as_slice())
    }

    fn merge_groups(&mut self) {
        let springs: Vec<Spring> = self.groups.iter().fold(vec![], |mut acc, g| {
            let mut springs = g.springs.clone();
            if !acc.is_empty() {
                springs.insert(0, Spring::Operational);
            }
            acc.append(&mut springs);
            acc
        });
        let required_indexes =
            springs
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (i, s)| {
                    if *s != Spring::Unknown {
                        acc.insert(i, *s);
                    }
                    acc
                });
        let length = springs.len();
        let group = Group {
            springs,
            required_indexes,
            length,
        };
        self.groups = Vec::from([group]);
    }

    fn unfolded(s: &str) -> Self {
        let (springs, group_nums) = s.split_once(' ').unwrap();
        let groups = (0..5)
            .fold(String::new(), |mut acc, _| {
                if !acc.is_empty() {
                    acc.push('?');
                }
                acc.push_str(springs);
                acc
            })
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let springs = s.chars().map(Spring::from).collect::<Vec<_>>();
                let required_indexes =
                    springs
                        .iter()
                        .enumerate()
                        .fold(HashMap::new(), |mut acc, (i, s)| {
                            if *s != Spring::Unknown {
                                acc.insert(i, *s);
                            }
                            acc
                        });
                let length = springs.len();
                Group {
                    springs,
                    required_indexes,
                    length,
                }
            })
            .collect::<Vec<_>>();
        let group_nums = group_nums
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .repeat(5);

        let mut row = Self { groups, group_nums };
        row.merge_groups();
        row
    }
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        let (springs, group_nums) = s.split_once(' ').unwrap();
        let groups = springs
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let springs = s.chars().map(Spring::from).collect::<Vec<_>>();
                let required_indexes =
                    springs
                        .iter()
                        .enumerate()
                        .fold(HashMap::new(), |mut acc, (i, s)| {
                            if *s != Spring::Unknown {
                                acc.insert(i, *s);
                            }
                            acc
                        });
                let length = springs.len();
                Group {
                    springs,
                    required_indexes,
                    length,
                }
            })
            .collect::<Vec<_>>();
        let group_nums = group_nums
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut row = Self { groups, group_nums };
        row.merge_groups();
        row
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_group_unknowns(length: usize) -> Group {
        Group {
            springs: vec![Spring::Unknown; length],
            required_indexes: HashMap::new(),
            length,
        }
    }

    #[test]
    fn test_singleton() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(1).combinations(&[1], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 1);
    }

    #[test]
    fn test_two() {
        {
            let current_combination = vec![];
            let mut all_combinations = 0;
            create_group_unknowns(2).combinations(&[1], current_combination, &mut all_combinations);
            assert_eq!(all_combinations, 2);
        }
    }

    #[test]
    fn test_three() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(3).combinations(&[1, 1], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 1);
    }

    #[test]
    fn test_three_one() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(3).combinations(&[1], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 3,);
    }

    #[test]
    fn test_four() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(4).combinations(&[2], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 3);
    }

    #[test]
    fn test_four_two() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(4).combinations(&[1, 1], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 3);
    }

    #[test]
    fn test_five() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(5).combinations(&[1, 2], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 3,);
    }

    #[test]
    fn test_six() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(7).combinations(&[1, 3], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 6,);
    }

    #[test]
    fn test_calc() {
        let group = Group {
            springs: vec![Spring::Damaged, Spring::Unknown, Spring::Unknown],
            required_indexes: HashMap::from_iter([(0, Spring::Damaged)]),
            length: 3,
        };

        assert_eq!(1, group.calculate(&[1]));

        let group = Group {
            springs: vec![
                Spring::Damaged,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Damaged,
            ],
            required_indexes: HashMap::from_iter([(0, Spring::Damaged), (3, Spring::Damaged)]),
            length: 4,
        };

        assert_eq!(1, group.calculate(&[1, 1]));

        let group = Group {
            springs: vec![
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Damaged,
            ],
            required_indexes: HashMap::from_iter([(3, Spring::Damaged)]),
            length: 4,
        };

        assert_eq!(2, group.calculate(&[1, 1]));
    }
}
