use std::{ffi::OsString, fs, process};

mod eight;
mod eleven;
mod five;
mod four;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
mod three;
mod two;
mod twelve;
mod fourteen;

const DATA_DIR: &str = "./data";

pub fn run() {
    let args: Vec<_> = std::env::args().collect();

    let day = args.get(1);
    let (data, day) = get_data(day);
    let (part_one, part_two) = solve(day, &data);
    println!("Part one: {}, part two: {}", part_one, part_two);
}

fn solve(day: u16, data: &str) -> (i128, i128) {
    match day {
        1 => (one::part_one(data).into(), one::part_two(data).into()),
        2 => (two::part_one(data).into(), two::part_two(data).into()),
        3 => (three::part_one(data).into(), three::part_two(data).into()),
        4 => (four::part_one(data).into(), four::part_two(data).into()),
        5 => (five::part_one(data).into(), five::part_two(data).into()),
        6 => (six::part_one(data).into(), six::part_two(data).into()),
        7 => (seven::part_one(data).into(), seven::part_two(data).into()),
        8 => (eight::part_one(data), eight::part_two(data)),
        9 => (nine::part_one(data).into(), nine::part_two(data).into()),
        10 => (ten::part_one(data), ten::part_two(data)),
        11 => (eleven::part_one(data), eleven::part_two(data)),
        12 => (twelve::part_one(data), twelve::part_two(data)),
        13 => (42, 42),
        14 => (fourteen::part_one(data), fourteen::part_two(data)),
        15 => (42, 42),
        16 => (42, 42),
        17 => (42, 42),
        18 => (42, 42),
        19 => (42, 42),
        20 => (42, 42),
        21 => (42, 42),
        22 => (42, 42),
        23 => (42, 42),
        24 => (42, 42),
        25 => (42, 42),
        _ => {
            panic!("Um...");
        }
    }
}

fn get_data(day: Option<&String>) -> (String, u16) {
    let is_test = day.is_some_and(|d| d.chars().last().is_some_and(|c| c == 't'));

    let entries: Vec<_> = std::fs::read_dir(DATA_DIR)
        .unwrap_or_else(|e| {
            panic!("ERROR: Failed to read {DATA_DIR} {e}");
        })
        .map(|entry| entry.expect("ERROR: Failed to read file"))
        .collect();

    if entries.is_empty() {
        eprintln!("No files found in {DATA_DIR}");
        process::exit(1);
    }

    let entry = match day {
        Some(day) => {
            let os_str_day = OsString::from(day);
            entries
                .iter()
                .find(|entry| os_str_day == entry.path().file_stem().expect("Invalid file format"))
                .unwrap_or_else(|| {
                    let last = entries.last().unwrap();
                    println!(
                        "Day {} not found in data files\nDefaulting to: {}",
                        day,
                        last.path().to_str().unwrap()
                    );
                    last
                })
        }
        None => entries.last().expect("No files found"),
    };

    let mut day = entry
        .path()
        .file_stem()
        .expect("Invalid file path")
        .to_str()
        .unwrap()
        .to_string();

    if is_test {
        day.pop();
    }

    let data = fs::read_to_string(entry.path()).expect("Failed to read day");
    (data, day.parse().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert(day_str: &str, day_num: u16, part_one: i128, part_two: i128) {
        let data = get_data(Some(&day_str.to_string()));
        let (part_one_ans, part_two_ans) = solve(day_num, data.0.as_str());
        assert_eq!(part_one, part_one_ans);
        assert_eq!(part_two, part_two_ans);
    }

    #[test]
    fn day_one() {
        assert("1", 1, 52974, 53340);
    }

    #[test]
    fn day_two() {
        assert("2", 2, 2771, 70924);
    }

    #[test]
    fn day_three() {
        assert("3", 3, 531561, 83279367);
    }

    #[test]
    fn day_four() {
        assert("4", 4, 23441, 5923918);
    }

    #[test]
    fn day_six() {
        assert("6", 6, 160816, 46561107);
    }

    #[test]
    fn day_seven() {
        assert("7", 7, 250058342, 250506580);
    }

    #[test]
    fn day_eight() {
        assert("8", 8, 20093, 22103062509257);
    }

    #[test]
    fn day_nine() {
        assert("9", 9, 1772145754, 867);
    }

    #[test]
    fn day_ten() {
        assert("10", 10, 6806, 449);
    }

    #[test]
    fn day_eleven() {
        assert("11", 11, 9565386, 857986849428);
    }
}
