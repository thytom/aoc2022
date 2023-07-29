use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

fn get_input_for_day(day: u8) -> Result<String, std::io::Error> {
    let cookie = std::env::var("AOC_SESSION_COOKIE").expect("Session cookie not set!");

    ureq::get(format!("https://adventofcode.com/2022/day/{}/input", day).as_str())
        .set("Cookie", format!("session={}", cookie).as_str())
        .call()
        .expect("Failed to fetch input!")
        .into_string()
}

fn perform_day(day: &Day) -> std::time::Duration {
    // Fetch the input
    println!("Fetching day {} input...", day.number);
    let input: String = get_input_for_day(day.number).expect("Failed to decode input!");

    let part1 = day.part1;
    let part2 = day.part2;

    let start_time = std::time::Instant::now();
    let p1: String = part1(&input);
    let p2: String = part2(&input);
    let total_time = start_time.elapsed();

    println!("Performing Day {}", day.number);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

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
