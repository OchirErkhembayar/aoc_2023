pub fn part_one(data: &str) -> i32 {
    let now = std::time::Instant::now();

    for _ in 0..50000 {
        part_one_inner(data);
    }

    let elapsed = now.elapsed();
    println!("Day 6 part 1: {}", elapsed.as_nanos() / 50000);
    part_one_inner(data)
}

fn part_one_inner(data: &str) -> i32 {
    let (times, distances) = data.split_once('\n').unwrap();
    let mut times = times.split_ascii_whitespace();
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

    for _ in 0..80000 {
        part_two_bytes(data);
    }

    let elapsed = now.elapsed();
    println!("Day 6 part 2: {}", elapsed.as_nanos() / 80000);
    assert!(part_two_bytes(data) == part_two_inner(data));
    part_two_bytes(data) as i32
}

pub fn part_two_inner(data: &str) -> f64 {
    let (times, distances) = data.split_once('\n').unwrap();
    let time: i64 = times
        .bytes()
        .skip(11)
        .rfold((0, 0), |(i, sum), c| {
            if c != 32 {
                (i + 1, sum + (c as i64 - '0' as i64) * 10_i64.pow(i as u32))
            } else {
                (i, sum)
            }
        })
        .1;
    let distance: i64 = distances
        .bytes()
        .skip(11)
        .rfold((0, 0), |(i, sum), c| {
            if c != 32 && c != 10 {
                (i + 1, sum + (c as i64 - '0' as i64) * 10_i64.pow(i as u32))
            } else {
                (i, sum)
            }
        })
        .1;

    let sqrt_p = f64::sqrt((time.pow(2) - 4 * (distance + 1)) as f64);
    (((time as f64 + sqrt_p) / 2.0) - ((time as f64 - sqrt_p) / 2.0)).floor()
}

fn part_two_bytes(data: &str) -> f64 {
    let mut bytes = data.bytes().rev();
    bytes.next();
    let mut next = bytes.next().unwrap();

    let mut distance = 0;
    let mut index = 0;
    while next != 58 {
        while next != 32 {
            distance += (next as i64 - 48) * 10_i64.pow(index);
            next = bytes.next().unwrap();
            index += 1;
        }
        next = bytes.next().unwrap();
    }
    bytes.next().unwrap();
    bytes.next().unwrap();
    bytes.next().unwrap();
    bytes.next().unwrap();
    bytes.next().unwrap();
    bytes.next().unwrap();
    bytes.next().unwrap();
    bytes.next().unwrap();
    bytes.next().unwrap();
    next = bytes.next().unwrap();

    let mut time = 0;
    index = 0;
    while next != 58 {
        while next != 32 {
            time += (next as i64 - 48) * 10_i64.pow(index);
            next = bytes.next().unwrap();
            index += 1;
        }
        next = bytes.next().unwrap();
    }

    let sqrt_p = f64::sqrt((time.pow(2) - 4 * (distance + 1)) as f64);
    (((time as f64 + sqrt_p) / 2.0) - ((time as f64 - sqrt_p) / 2.0)).floor()
}
