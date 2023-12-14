use std::{
    char,
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

fn part_one_inner(data: &str) -> i32 {
    data.lines().map(|l| Row::from(l).calculate()).sum()
}

pub fn part_two(data: &str) -> i128 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);
    let elapsed = now.elapsed();
    println!("Day 12 part 2: {}", elapsed.as_secs_f64());
    ans
}

fn part_two_inner(data: &str) -> i128 {
    let rows = data.lines().map(Row::unfolded).collect::<Vec<_>>();
    println!("Rows: {:?}", rows);

    let sum = Arc::new(Mutex::new((0, 1)));
    std::thread::scope(|s| {
        for row in rows {
            let sum = Arc::clone(&sum);
            s.spawn(move || {
                let comb = row.calculate();
                let mut s = sum.lock().unwrap();
                (*s).0 += comb as i128;
                (*s).1 += 1;
                println!("s: {}", (*s).1);
                drop(s);
            });
        }
    });
    let s = sum.lock().unwrap();
    (*s).0
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Row {
    groups: Vec<Group>,
    group_nums: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Group {
    springs: Vec<Spring>,
    required_indexes: Vec<(usize, Spring)>,
}

impl Group {
    fn calculate(&self, groupings: &[usize]) -> i32 {
        let mut combinations = 0;
        self.get_combinations(groupings, vec![], &mut combinations);
        println!("Calced: {combinations}");
        combinations
    }

    fn get_combinations(
        &self,
        groupings: &[usize],
        mut current_combination: Vec<Spring>,
        combos: &mut i32,
    ) {
        let length = self.springs.len();
        if self
            .required_indexes
            .iter()
            .any(|(i, kind)| current_combination.get(*i).is_some_and(|s| *s != *kind))
        {
            return;
        }
        if groupings.is_empty() {
            while current_combination.len() < length {
                current_combination.push(Spring::Operational);
            }
            if self
                .required_indexes
                .iter()
                .all(|(i, kind)| current_combination[*i] == *kind)
            {
                *combos += 1;
            }
            return;
        }

        let min_width: usize = groupings.iter().sum::<usize>() + groupings.len() - 1;
        let max_len = length - current_combination.len() - min_width;

        for i in 0..=max_len {
            let mut temp_comb = current_combination.clone();
            temp_comb.append(&mut vec![Spring::Operational; i]);
            temp_comb.append(&mut vec![Spring::Damaged; groupings[0]]);
            if temp_comb.len() < length {
                temp_comb.push(Spring::Operational);
            }
            if self
                .required_indexes
                .iter()
                .any(|(i, kind)| current_combination.get(*i).is_some_and(|s| *s != *kind))
            {
                continue;
            }
            self.get_combinations(&groupings[1..], temp_comb, combos);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
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
    fn calculate(self) -> i32 {
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
        let required_indexes = springs.iter().enumerate().fold(vec![], |mut acc, (i, s)| {
            if *s != Spring::Unknown {
                acc.push((i, *s));
            }
            acc
        });
        let group = Group {
            springs,
            required_indexes,
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
                    springs.iter().enumerate().fold(vec![], |mut acc, (i, s)| {
                        if *s != Spring::Unknown {
                            acc.push((i, *s));
                        }
                        acc
                    });
                Group {
                    springs,
                    required_indexes,
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
                    springs.iter().enumerate().fold(vec![], |mut acc, (i, s)| {
                        if *s != Spring::Unknown {
                            acc.push((i, *s));
                        }
                        acc
                    });
                Group {
                    springs,
                    required_indexes,
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
            required_indexes: vec![],
        }
    }

    #[test]
    fn test_singleton() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(1).get_combinations(&[1], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 1);
    }

    #[test]
    fn test_two() {
        {
            let current_combination = vec![];
            let mut all_combinations = 0;
            create_group_unknowns(2).get_combinations(
                &[1],
                current_combination,
                &mut all_combinations,
            );
            assert_eq!(all_combinations, 2);
        }
    }

    #[test]
    fn test_three() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(3).get_combinations(
            &[1, 1],
            current_combination,
            &mut all_combinations,
        );
        assert_eq!(all_combinations, 1);
    }

    #[test]
    fn test_three_one() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(3).get_combinations(&[1], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 3,);
    }

    #[test]
    fn test_four() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(4).get_combinations(&[2], current_combination, &mut all_combinations);
        assert_eq!(all_combinations, 3);
    }

    #[test]
    fn test_four_two() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(4).get_combinations(
            &[1, 1],
            current_combination,
            &mut all_combinations,
        );
        assert_eq!(all_combinations, 3);
    }

    #[test]
    fn test_five() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(5).get_combinations(
            &[1, 2],
            current_combination,
            &mut all_combinations,
        );
        assert_eq!(all_combinations, 3,);
    }

    #[test]
    fn test_six() {
        let current_combination = vec![];
        let mut all_combinations = 0;
        create_group_unknowns(7).get_combinations(
            &[1, 3],
            current_combination,
            &mut all_combinations,
        );
        assert_eq!(all_combinations, 6,);
    }

    #[test]
    fn test_calc() {
        let group = Group {
            springs: vec![Spring::Damaged, Spring::Unknown, Spring::Unknown],
            required_indexes: vec![(0, Spring::Damaged)],
        };

        assert_eq!(1, group.calculate(&[1]));

        let group = Group {
            springs: vec![
                Spring::Damaged,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Damaged,
            ],
            required_indexes: vec![(0, Spring::Damaged), (3, Spring::Damaged)],
        };

        assert_eq!(1, group.calculate(&[1, 1]));

        let group = Group {
            springs: vec![
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Damaged,
            ],
            required_indexes: vec![(3, Spring::Damaged)],
        };

        assert_eq!(2, group.calculate(&[1, 1]));
    }
}
