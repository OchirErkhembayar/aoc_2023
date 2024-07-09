use std::collections::HashMap;

pub fn part_one(data: &str) -> i128 {
    data.trim()
        .split(',')
        .map(|step| {
            step.chars().fold(0, |mut acc, c| {
                let c = c as u8 as i128;
                acc += c;
                acc *= 17;
                acc % 256
            })
        })
        .sum()
}

#[derive(Debug, Clone)]
enum Instr {
    Insert((String, u8)),
    Remove(String),
}

impl Instr {
    fn from(a: &str, b: &str) -> Self {
        match b {
            "" => Instr::Remove(a.to_string()),
            _ => Instr::Insert((a.to_string(), b.parse().unwrap())),
        }
    }
}

pub fn part_two(data: &str) -> i128 {
    let boxes: HashMap<i32, Vec<(String, u8)>> = HashMap::from_iter((0..=255).map(|i| (i, vec![])));
    data.trim()
        .split(',')
        .map(|step| {
            let (a, b) = step.split_once(&['=', '-']).unwrap();
            (
                a.chars().fold(0, |mut acc, c| {
                    let c = c as u8 as i128;
                    acc += c;
                    acc *= 17;
                    acc % 256
                }),
                Instr::from(a, b),
            )
        })
        .fold(boxes, |mut boxes, (num, instr)| {
            let lenses = boxes
                .get_mut(&(num as i32))
                .expect(&format!("Box {} not found", num));
            match instr {
                Instr::Insert((name, length)) => {
                    if let Some(i) = lenses.iter().position(|(n, _)| *n == name) {
                        let lense = lenses.get_mut(i).unwrap();
                        lense.1 = length;
                    } else {
                        lenses.push((name, length));
                    }
                }
                Instr::Remove(name) => {
                    if let Some(i) = lenses.iter().position(|(n, _)| *n == name) {
                        lenses.remove(i);
                    }
                }
            };
            boxes
        })
        .into_iter()
        .map(|(bi, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(i, (_, f))| ((bi as usize + 1) * (i + 1) * f as usize) as i128)
                .sum::<i128>()
        })
        .sum()
}
