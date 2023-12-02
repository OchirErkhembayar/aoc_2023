pub fn part_one(data: &str) -> i32 {
    let mut id_sum: i32 = 0;
    for line in data.lines() {
        let parts: Vec<_> = line.split(":").collect();
        let id: i32 = parts[0].split(' ').next_back().unwrap().parse().unwrap();

        let games: Vec<_> = parts[1]
            .split(';')
            .map(|g| {
                g.trim()
                    .split(", ")
                    .map(|m| m.split(' ').collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut valid = true;
        for game in games {
            for round in game {
                let colour = round[1];
                let number = round[0].parse::<i32>().unwrap();
                let too_many = match colour {
                    "red" => number > 12,
                    "green" => number > 13,
                    "blue" => number > 14,
                    _ => unimplemented!("Wtf"),
                };
                if too_many {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            id_sum += id;
        }
    }

    id_sum
}

pub fn part_two(data: &str) -> i32 {
    let mut power_sum: i32 = 0;
    for line in data.lines() {
        let mut red_limit = 0;
        let mut green_limit = 0;
        let mut blue_limit = 0;

        let games: Vec<_> = line.split(":").collect::<Vec<_>>()[1]
            .split(';')
            .map(|g| {
                g.trim()
                    .split(", ")
                    .map(|m| m.split(' ').collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .collect();

        for game in games {
            for round in game {
                let number = round[0].parse::<i32>().unwrap();
                let colour = round[1];
                match colour {
                    "red" => {
                        if number > red_limit {
                            red_limit = number;
                        }
                    }
                    "green" => {
                        if number > green_limit {
                            green_limit = number;
                        }
                    }
                    "blue" => {
                        if number > blue_limit {
                            blue_limit = number;
                        }
                    }
                    _ => unimplemented!("Wtf"),
                };
            }
        }

        power_sum += red_limit * green_limit * blue_limit;
    }

    power_sum
}
