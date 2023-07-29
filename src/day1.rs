fn input_to_snack_counts(input: &String) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|str: &str| -> u32 {
            str.strip_suffix("\n")
                .unwrap_or(str)
                .split("\n")
                .map(|num| -> u32 { num.parse().expect("Couldn't parse num!") })
                .sum()
        })
        .collect()
}

pub fn part1(input: &String) -> String {
    let nums: Vec<u32> = input_to_snack_counts(input);

    format!("{}", nums.iter().max().unwrap())
}

pub fn part2(input: &String) -> String {
    let mut nums: Vec<u32> = input_to_snack_counts(input);

    nums.sort();

    format!("{}", nums.iter().rev().take(3).sum::<u32>())
}

#[cfg(test)]
mod tests {
    pub const DAY1_PART1_TEST: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    pub const DAY1_PART1_TEST_EXPECT: &str = "24000";
    pub const DAY1_PART2_TEST_EXPECT: &str = "45000";

    use super::*;

    #[test]
    pub fn test_day1() {
        assert_eq!(
            part1(&DAY1_PART1_TEST.to_owned()).as_str(),
            DAY1_PART1_TEST_EXPECT
        )
    }

    #[test]
    pub fn test_day2() {
        assert_eq!(
            part2(&DAY1_PART1_TEST.to_owned()).as_str(),
            DAY1_PART2_TEST_EXPECT
        )
    }
}
