pub fn part_one(data: &str) -> i32 {
    data.lines()
        .map(|line| {
            let nums = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
            let first = nums.first().unwrap().to_owned();
            let last = nums.last().unwrap().to_owned();
            let mut number = String::new();
            number.push(first);
            number.push(last);
            number.parse::<i32>().unwrap()
        })
        .sum()
}

pub fn part_two(data: &str) -> i32 {
    data.lines()
        .map(|line| {
            let chars: Vec<_> = line.chars().collect();
            let mut digits: Vec<i32> = Vec::new();
            for (i, c) in chars.iter().enumerate() {
                if c.is_numeric() {
                    digits.push(c.to_string().parse::<i32>().unwrap());
                } else if let Some(num) = is_numeric_str(&chars[i..]) {
                    digits.push(num);
                }
            }
            let first = digits
                .first()
                .unwrap_or_else(|| panic!("Failed to get digits from {:?}", chars));
            let second = digits.last().unwrap_or(first);
            first * 10 + second
        })
        .sum()
}

fn is_numeric_str(chars: &[char]) -> Option<i32> {
    if chars.len() < 3 {
        return None;
    }

    match chars[0] {
        'o' => {
            if chars[0..3] == ['o', 'n', 'e'] {
                Some(1)
            } else {
                None
            }
        }
        't' => {
            // two
            // three
            if chars[0..3] == ['t', 'w', 'o'] {
                Some(2)
            } else if chars.len() >= 5 && chars[0..5] == ['t', 'h', 'r', 'e', 'e'] {
                Some(3)
            } else {
                None
            }
        }
        'f' => {
            // four five
            if chars.len() < 4 {
                None
            } else if chars[0..4] == ['f', 'o', 'u', 'r'] {
                Some(4)
            } else if chars[0..4] == ['f', 'i', 'v', 'e'] {
                Some(5)
            } else {
                None
            }
        }
        's' => {
            // six seven
            if chars[0..3] == ['s', 'i', 'x'] {
                Some(6)
            } else if chars.len() >= 5 && chars[0..5] == ['s', 'e', 'v', 'e', 'n'] {
                Some(7)
            } else {
                None
            }
        }
        'e' => {
            if chars.len() >= 5 && chars[0..5] == ['e', 'i', 'g', 'h', 't'] {
                Some(8)
            } else {
                None
            }
        }
        'n' => {
            if chars.len() >= 4 && chars[0..4] == ['n', 'i', 'n', 'e'] {
                Some(9)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_num() {
        assert_eq!(Some(1), is_numeric_str(&['o', 'n', 'e', 'f']));
        assert_eq!(Some(3), is_numeric_str(&['t', 'h', 'r', 'e', 'e', 'f']));
        assert_eq!(Some(1), is_numeric_str(&['o', 'n', 'e', 'f']));
        assert_eq!(Some(1), is_numeric_str(&['o', 'n', 'e', 'f']));
    }
}
