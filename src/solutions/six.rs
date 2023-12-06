pub fn part_one(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_one_inner(data);

    let elapsed = now.elapsed();
    println!("Day 6 part 1: {}", elapsed.as_micros());
    ans
}

fn part_one_inner(data: &str) -> i32 {
    let (times, distances) = data.split_once('\n').unwrap();
    let mut times = times.split_whitespace();
    times.next().unwrap();
    let times = times.map(|t| t.parse::<i32>().unwrap());
    let mut distances = distances.split_ascii_whitespace();
    distances.next().unwrap();
    let distances = distances
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let tds = times.zip(distances);

    tds.fold(1, |total, (t, d)| {
        // dist = s * (t - rt) (speed, remaining time)
        // s = pt (press time)
        // dist = pt * (t - pt)
        // dist = pt . t - pt . pt
        // dist > d
        let possible_pts = 0..=t;
        let allowed_pts = possible_pts.filter(|pt| (pt * t - pt.pow(2)) > d).count() as i32;
        total * allowed_pts
    })
}

pub fn part_two(data: &str) -> i32 {
    let now = std::time::Instant::now();
    let ans = part_two_inner(data);

    let elapsed = now.elapsed();
    println!("Day 6 part 2: {}", elapsed.as_micros());
    ans
}

pub fn part_two_inner(data: &str) -> i32 {
    let (times, distances) = data.split_once('\n').unwrap();
    let time = {
        let mut times = times.split_whitespace();
        times.next().unwrap();
        times.collect::<Vec<_>>().join("").parse::<i64>().unwrap()
    };
    let distance = {
        let mut distances = distances.split_ascii_whitespace();
        distances.next().unwrap();
        distances
            .collect::<Vec<_>>()
            .join("")
            .parse::<i64>()
            .unwrap()
    };

    let max_t = ((time as f64 + f64::sqrt((time.pow(2) - 4 * (distance + 1)) as f64)) / 2.0).floor() as i64;
    let min_t = ((time as f64 - f64::sqrt((time.pow(2) - 4 * (distance + 1)) as f64)) / 2.0).floor() as i64;

    (max_t - min_t) as i32
}
