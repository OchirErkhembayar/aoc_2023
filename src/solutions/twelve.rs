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
    // Find the combinations we can get if every spring was ?
    // Then filter those to the ones where the # exists in their position
    fn calculate(&self, groupings: Vec<i32>) -> i32 {
        let groupings_len = groupings.len() as i32;
        let springs_len = self.springs.len() as i32;
        if groupings_len == 1 && springs_len == groupings[0] {
            return groupings[0];
        }
        // ???#? 3
        // required pos == 4
        // 4 - 3 combinations don't wory
        //
        // ??#??? 3
        // required pos == 3
        // (len - 3) - 3 + 1 don't work
        //
        // ???#??????? 3 (len == 11)
        // required pos == 4
        // 4 - 3 from front don't work
        // (len - 4) - 3 + 1 don't work from back (5)
        //
        // ??#? 3
        // 3 - 3 don't work from front ( <= 0 so all work)
        // (len - 3) - 3 + 1 ( <= 0 so all work)
        //
        // ??#?# 3 Only 1...
        //
        // Find highest index we can start from
        // Create vector of possible ranges
        // Filter them to the ranges which have the required indexes
        //
        // Then look at multiple groupings
        // ?????? 1, 2
        // #.##??
        // #..##?
        // #...##
        // .#.##.
        // .#..##
        // ..#.##
        //
        // Put the first grouping in position 1
        // Find possible combinations for the second one
        // Move up once, repeat until 1 can't move up anymore
        //
        // ???????? 1, 2, 1
        // #.##.#..
        // #.##..#.
        // #.##...#
        // #..##.#.
        // #..##..#
        // #...##.#
        // .#.##.#.
        // .#..##.#
        // ..#.##.#
        //
        // There is a minimum range that they will take
        // Put the first one at the start of the range and calc then shift up once
        //
        let required_indexes = self
            .springs
            .iter()
            .enumerate()
            .filter_map(|(i, s)| if *s == Spring::Damaged { Some(i) } else { None })
            .collect::<Vec<_>>();

        let min_width = groupings.iter().fold(0, |acc, d| acc + d) + groupings_len - 1;

        // indexes of damaged springs
        let mut iterations: Vec<Vec<i32>> = vec![];

        let mut start_index = 0;
        while start_index + min_width <= springs_len {
            start_index += 1;
        }

        42
    }

    /*
     * ??????? 1, 2, 1
     * len = 7
     * num_groups = 3
     * groupings = [1, 2, 1]
     */
    fn get_combinations(
        &self,
        length: usize,
        groupings: &[usize],
        current_combination: &mut Vec<Spring>,
        all_combinations: &mut Vec<Vec<Spring>>,
    ) {
        if groupings.is_empty() {
            while current_combination.len() < self.springs.len() {
                current_combination.push(Spring::Operational);
            }
            all_combinations.push(current_combination.clone());
            current_combination.clear();
            return;
        }

        let num_groups = groupings.len();
        let min_width: usize = groupings.iter().sum::<usize>() + num_groups - 1;

        for i in 0..=(length - min_width) {
            current_combination.append(&mut vec![Spring::Operational; i]);
            current_combination.append(&mut vec![Spring::Damaged; groupings[0]]);
            current_combination.push(Spring::Operational);
            // This should clone and add a grouping to all combinations
            self.get_combinations(
                length - groupings[0] - i,
                &groupings[1..],
                current_combination,
                all_combinations,
            );
        }
    }

    fn generate_combinations(
        &self,
        num_groups: i32,
        group_lens: &[i32],
        mut current_combination: Vec<Spring>,
    ) -> Vec<Vec<Spring>> {
        if num_groups == 0 {
            return vec![current_combination];
        }

        let combinations = vec![];
        // ????? 2, 2
        // 5 - 4 - (2 - 1) + 1 = 1
        for i in
            0..(self.springs.len() as i32 - group_lens.iter().sum::<i32>() - (num_groups - 1) + 1)
        {
            // Unless we're the first one we want to push onto it
            for _idx in 0..=i {
                current_combination.push(Spring::Operational);
            }
        }
        combinations
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_groups() {
        let group = Group {
            springs: vec![
                Spring::Unknown,
            ],
        };

        let mut current_combination = vec![];
        let mut all_combinations = vec![];

        group.get_combinations(4, &[1], &mut current_combination, &mut all_combinations);
        assert_eq!(all_combinations, vec![vec![Spring::Damaged]]);
    }
}
