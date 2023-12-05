mod four;
mod one;
mod three;
mod two;
mod five;

pub fn solve(day: u16, data: &str) -> (i32, i32) {
    match day {
        1 => (one::part_one(data), one::part_two(data)),
        2 => (two::part_one(data), two::part_two(data)),
        3 => (three::part_one(data), three::part_two(data)),
        4 => (four::part_one(data), four::part_two(data)),
        5 => (five::part_one(data), five::part_two(data)),
        6 => (42, 42),
        7 => (42, 42),
        8 => (42, 42),
        9 => (42, 42),
        10 => (42, 42),
        11 => (42, 42),
        12 => (42, 42),
        13 => (42, 42),
        14 => (42, 42),
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
