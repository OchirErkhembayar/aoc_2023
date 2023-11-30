use std::{ffi::OsString, fs, process};

const DATA_DIR: &str = "./data";

mod solutions;

fn main() {
    let (data, day) = get_data();
    println!("Result: {}", solutions::solve(day, data));
}

fn get_data() -> (String, u16) {
    let args: Vec<_> = std::env::args().collect();

    let day = args.get(1);

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

    let day: u16 = entry
        .path()
        .file_stem()
        .expect("Invalid file path")
        .to_str()
        .unwrap()
        .parse()
        .expect("Non numeric file name");
    let data = fs::read_to_string(entry.path()).expect("Failed to read day");
    (data, day)
}
