mod day1;

fn get_input_for_day(day: u8) -> Result<String, std::io::Error> {
    let cookie = std::env::var("AOC_SESSION_COOKIE").expect("Session cookie not set!");

    ureq::get(format!("https://adventofcode.com/2022/day/{}/input", day).as_str())
        .set("Cookie", format!("session={}", cookie).as_str())
        .call()
        .expect("Failed to fetch input!")
        .into_string()
}

fn perform_day(day: Day) {
    // Fetch the input
    let input: String = get_input_for_day(day.number).expect("Failed to decode input!");

    let part1 = day.part1;
    let part2 = day.part2;

    println!("Performing Day {}", day.number);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Day {
    number: u8,
    part1: fn(&String) -> String,
    part2: fn(&String) -> String,
}

fn main() {
    perform_day(Day {
        number: 1,
        part1: day1::part1,
        part2: day1::part2,
    })
}
