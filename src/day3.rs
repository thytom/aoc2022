use std::ops::Deref;

fn char_to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Bad char received!"),
    }
}

fn find_commonality(rs: &[&str]) -> char {
    let first = rs.first().expect("Passed empty vec!");
    let rest = rs[1..].to_vec();

    for c in first.chars() {
        if rest.iter().all(|v| -> bool { v.contains(c) }) {
            return c;
        }
    }

    return 0 as char;
}

pub fn part1(input: &String) -> String {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .map(|s| -> Vec<&str> {
            let (f, l) = s.split_at(s.len() / 2);
            vec![f, l]
        })
        .map(|v| -> char { find_commonality(v.deref()) })
        .map(char_to_priority)
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &String) -> String {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(find_commonality)
        .map(char_to_priority)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    pub const TEST: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    pub const PART1_TEST_EXPECT: &str = "157";
    pub const PART2_TEST_EXPECT: &str = "70";

    use super::*;

    #[test]
    pub fn test_day1() {
        assert_eq!(part1(&TEST.to_owned()).as_str(), PART1_TEST_EXPECT)
    }

    #[test]
    pub fn test_day2() {
        assert_eq!(part2(&TEST.to_owned()).as_str(), PART2_TEST_EXPECT)
    }
}
