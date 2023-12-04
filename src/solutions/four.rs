pub fn part_one(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);

    let elapsed = now.elapsed();
    println!("Day 4 part 1: {}", elapsed.as_micros());
    ans
}

fn part_one_inner(data: &str) -> i32 {
    let mut total = 0;
    for line in data.lines() {
        let cards = line.split_once(':').unwrap().1;
        let (winning, cards) = cards.split_once('|').unwrap();
        let winners: Vec<_> = winning.split(' ').collect();
        let only_winning = cards
            .split(' ')
            .filter(|c| !c.is_empty() && winners.contains(c))
            .count();
        if only_winning > 0 {
            total += i32::pow(2, (only_winning - 1) as u32);
        }
    }
    total
}

pub fn part_two(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);

    let elapsed = now.elapsed();
    println!("Day 4 part 2: {}", elapsed.as_micros());
    ans
}

fn part_two_inner(data: &str) -> i32 {
    let count = data.lines().count();
    let mut copies = vec![1; count];

    for (i, line) in data.lines().enumerate() {
        let cards = line.split_once(':').unwrap().1;
        let (winning, cards) = cards.split_once('|').unwrap();
        let winners: Vec<_> = winning.split(' ').collect();
        let only_winning = cards
            .split(' ')
            .filter(|c| !c.is_empty() && winners.contains(c))
            .count();
        for index in (i + 1)..=(i + only_winning) {
            copies[index] += copies[i];
        }
    }

    copies.iter().sum()
}
