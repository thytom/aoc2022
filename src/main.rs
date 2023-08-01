use std::env;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::result::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn get_input_for_day(day: u8) -> Result<String, std::io::Error> {
    let cookie = std::env::var("AOC_SESSION_COOKIE").expect("Session cookie not set!");

    ureq::get(format!("https://adventofcode.com/2022/day/{}/input", day).as_str())
        .set("Cookie", format!("session={}", cookie).as_str())
        .call()
        .expect("Failed to fetch input!")
        .into_string()
}

fn get_input_from_file(day_num: u8) -> Result<String, std::io::Error> {
    read_to_string(format!("./cache/day{}.txt", day_num).as_str())
}

fn write_file_cache(day_num: u8, input: &str) {
    if !Path::new("./cache").exists() {
        println!("Creating cache dir ./cache");
        assert!(std::fs::create_dir("./cache").is_ok());
    }

    println!("Writing input for day {} to cache...", day_num);

    let mut file = File::create(Path::new(format!("./cache/day{}.txt", day_num).as_str()))
        .expect("Couldn't create file!");

    file.write(input.as_bytes())
        .expect("Failed to write to file!");
}

fn perform_day(day: &Day) -> std::time::Duration {
    let file_input = get_input_from_file(day.number);
    let input: String;

    if file_input.is_err() {
        input = get_input_for_day(day.number).expect("Failed to decode input!");
        write_file_cache(day.number, &input);
    } else {
        println!("Found cached input for day {}", day.number);
        input = file_input.unwrap();
    }

    let part1 = day.part1;
    let part2 = day.part2;

    let start_time = std::time::Instant::now();
    let p1: String = part1(&input);
    let p1_time = start_time.elapsed();

    let start2_time = std::time::Instant::now();
    let p2: String = part2(&input);
    let p2_time = start2_time.elapsed();

    println!("Performing Day {}", day.number);
    println!("Part 1: {} in {} us", p1, p1_time.as_micros());
    println!("Part 2: {} in {} us", p2, p2_time.as_micros());

    let total_time = p1_time + p2_time;

    println!(
        "Day {} completed in {} us",
        day.number,
        total_time.as_micros()
    );

    total_time
}

#[derive(Debug)]
struct Day {
    number: u8,
    part1: fn(&String) -> String,
    part2: fn(&String) -> String,
}

fn main() {
    let should_run_all = env::args().find(|e| e == "all");

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let days: Vec<Day> = vec![
        Day { number: 1, part1: day1::part1, part2: day1::part2, },
        Day { number: 2, part1: day2::part1, part2: day2::part2, },
        Day { number: 3, part1: day3::part1, part2: day3::part2, },
        Day { number: 4, part1: day4::part1, part2: day4::part2, },
        Day { number: 5, part1: day5::part1, part2: day5::part2, },
        // Day { number: X, part1: dayX::part1, part2: dayX::part2, },
    ];

    let prog_time;

    match should_run_all {
        Some(_) => {
            prog_time = days.iter().map(perform_day).sum::<std::time::Duration>();
        }

        _ => {
            prog_time = perform_day(days.iter().last().unwrap());
        }
    }

    println!(
        "\n{} days completed in {} ms",
        days.len(),
        prog_time.as_millis()
    );
}
