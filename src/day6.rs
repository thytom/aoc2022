use std::collections::HashSet;

fn check_seq(v: &[u8]) -> bool {
    let mut map = HashSet::new();

    for c in v {
        map.insert(c);
    }

    map.len() == v.len()
}

fn check_str_chunks(input: &String, size: usize) -> String {
    let bytes = input.as_bytes();

    for i in 0..bytes.len() - size {
        if check_seq(&bytes[i..i + size]) {
            return (i + size).to_string();
        }
    }

    "".to_owned()
}

pub fn part1(input: &String) -> String {
    check_str_chunks(input, 4)
}

pub fn part2(input: &String) -> String {
    check_str_chunks(input, 14)
}

#[cfg(test)]
mod tests {
    pub const TEST: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    pub const PART1_TEST_EXPECT: [&str; 5] = ["7", "5", "6", "10", "11"];
    pub const PART2_TEST_EXPECT: [&str; 5] = ["19", "23", "23", "29", "26"];

    use super::*;

    #[test]
    pub fn test_day1() {
        for i in 0..5 {
            assert_eq!(part1(&TEST[i].to_owned()).as_str(), PART1_TEST_EXPECT[i]);
        }
    }

    #[test]
    pub fn test_day2() {
        for i in 0..5 {
            assert_eq!(part2(&TEST[i].to_owned()).as_str(), PART2_TEST_EXPECT[i]);
        }
    }
}
