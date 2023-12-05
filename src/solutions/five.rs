pub fn part_one(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);
    let elapsed = now.elapsed();
    println!("Day 5 part 1: {}", elapsed.as_micros());
    ans
}

fn part_one_inner(data: &str) -> i32 {
    let mut split = data.split("\n\n");
    let seeds = split.next().unwrap();
    let mut seeds = seeds
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    for line in split {
        let mut split = line.trim().split('\n');
        split.next().unwrap();
        let mut conversions = vec![];
        for ranges in split {
            let ranges = ranges.split_whitespace().collect::<Vec<_>>();
            let end: i64 = ranges[0].parse().unwrap();
            let start: i64 = ranges[1].parse().unwrap();
            let length: i64 = ranges[2].parse().unwrap();
            // subtract conversion to get it
            let conversion = start - end;
            let range = start..(start + length);
            conversions.push((conversion, range));
        }
        for seed in seeds.iter_mut() {
            if let Some(conversion) = conversions.iter().find(|(_, r)| r.contains(seed)) {
                *seed -= conversion.0;
            }
        }
    }

    *seeds.iter().min().unwrap() as i32
}

pub fn part_two(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);
    let elapsed = now.elapsed();
    println!("Day 5 part 1: {}", elapsed.as_micros());
    ans
}

// EXTREMELY SLOW. Need to refactor
fn part_two_inner(data: &str) -> i32 {
    let mut split = data.split("\n\n");
    let seeds = split.next().unwrap();
    let seeds = seeds
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let ranges: Vec<_> = seeds.chunks(2).collect();
    let mut seeds = vec![];
    for pair in ranges {
        let start = pair[0];
        let range = pair[1];
        for s in start..(start + range) {
            seeds.push(s);
        }
    }
    for line in split {
        let mut split = line.trim().split('\n');
        split.next().unwrap();
        let mut conversions = vec![];
        for ranges in split {
            let ranges = ranges.split_whitespace().collect::<Vec<_>>();
            let end: i64 = ranges[0].parse().unwrap();
            let start: i64 = ranges[1].parse().unwrap();
            let length: i64 = ranges[2].parse().unwrap();
            // subtract conversion to get it
            let conversion = start - end;
            let range = start..(start + length);
            conversions.push((conversion, range));
        }
        for seed in seeds.iter_mut() {
            if let Some(conversion) = conversions.iter().find(|(_, r)| r.contains(seed)) {
                *seed -= conversion.0;
            }
        }
    }

    *seeds.iter().min().unwrap() as i32
}
