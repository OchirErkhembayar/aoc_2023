pub fn part_one(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = data.lines().fold(0, |acc, line| {
        let (identifier, games) = line.split_once(':').unwrap();

        if !games
            .split(';')
            .map(|g| g.trim().split(", ").map(|m| m.split_once(' ').unwrap()))
            .any(|mut g| {
                g.any(|r| {
                    let number: i32 = r.0.parse().unwrap();

                    match r.1 {
                        "red" => number > 12,
                        "green" => number > 13,
                        "blue" => number > 14,
                        _ => unimplemented!("Wtf"),
                    }
                })
            })
        {
            acc + identifier
                .split_once(' ')
                .unwrap()
                .1
                .parse::<i32>()
                .unwrap()
        } else {
            acc
        }
    });

    let elapsed = now.elapsed();
    println!("Day 2 part 1: {:?}", elapsed.as_micros());
    ans
}

pub fn part_two(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = data.lines().fold(0, |acc, line| {
        let (r, g, b) = line
            .split_once(':')
            .unwrap()
            .1
            .split(';')
            .map(|g| g.trim().split(", ").map(|m| m.split_once(' ').unwrap()))
            .fold((0, 0, 0), |(r, g, b), game| {
                game.fold((r, g, b), |(acc_r, acc_g, acc_b), r| {
                    let number: i32 = r.0.parse().unwrap();
                    match r.1 {
                        "red" => {
                            if number > acc_r {
                                (number, acc_g, acc_b)
                            } else {
                                (acc_r, acc_g, acc_b)
                            }
                        }
                        "green" => {
                            if number > acc_g {
                                (acc_r, number, acc_b)
                            } else {
                                (acc_r, acc_g, acc_b)
                            }
                        }
                        "blue" => {
                            if number > acc_b {
                                (acc_r, acc_g, number)
                            } else {
                                (acc_r, acc_g, acc_b)
                            }
                        }
                        _ => unimplemented!("Wtf"),
                    }
                })
            });

        acc + r * g * b
    });

    let elapsed = now.elapsed();
    println!("Day 2 part 2: {:?}", elapsed.as_micros());
    ans
}
