pub fn part_one(data: &str) -> i32 {
    let mut nums: Vec<Num> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();
    for (row, line) in data.lines().enumerate() {
        let mut digits: Vec<(usize, char)> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                digits.push((i, c));
            } else {
                if c != '.' {
                    gears.push(Gear { index: i, row })
                }
                if !digits.is_empty() {
                    nums.push(Num::new(digits.clone(), row));
                    digits = Vec::new();
                }
            }
        }
        if !digits.is_empty() {
            nums.push(Num::new(digits.clone(), row));
        }
    }
    nums.iter()
        .filter_map(|num| {
            if gears.iter().any(|g| num.touches(g)) {
                Some(num.num)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_two(data: &str) -> i32 {
    let mut nums: Vec<Num> = Vec::new();
    let mut characters: Vec<Gear> = Vec::new();
    for (row, line) in data.lines().enumerate() {
        let mut digits: Vec<(usize, char)> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                digits.push((i, c));
            } else {
                if c == '*' {
                    characters.push(Gear { index: i, row })
                }
                if !digits.is_empty() {
                    nums.push(Num::new(digits.clone(), row));
                    digits = Vec::new();
                }
            }
        }
        if !digits.is_empty() {
            nums.push(Num::new(digits.clone(), row));
        }
    }
    characters
        .iter_mut()
        .filter_map(|c| {
            let mut touches = 0;
            let mut gears = 1;
            for num in &nums {
                if num.touches(c) {
                    touches += 1;
                    gears *= num.num;
                }
            }
            if touches == 2 {
                Some(gears)
            } else {
                None
            }
        })
        .sum()
}

#[derive(Debug)]
struct Gear {
    index: usize,
    row: usize,
}

#[derive(Debug)]
struct Num {
    num: i32,
    start: usize,
    end: usize,
    row: usize,
}

impl Num {
    fn new(digits: Vec<(usize, char)>, row: usize) -> Self {
        let start = digits.first().unwrap().0;
        let end = digits.last().unwrap().0;
        let num = digits.into_iter().map(|(_, d)| d).collect::<Vec<_>>();
        let mut num_str = String::new();
        for n in num {
            num_str.push(n);
        }
        let num = num_str.parse().unwrap();
        Self {
            num,
            start,
            end,
            row,
        }
    }

    fn touches(&self, ch: &Gear) -> bool {
        if self.row.abs_diff(ch.row) > 1 {
            return false;
        }

        let gear_pos = ch.index as i32;
        let start_pos = self.start as i32;
        let end_pos = self.end as i32;

        (gear_pos - end_pos <= 1) && (start_pos - gear_pos <= 1)
    }
}
