pub fn part1(input: &String) -> String {
    let groups = input.split("\n\n");

    let max: u32 = groups
        .map(|str: &str| -> u32 {
            str.strip_suffix("\n")
                .unwrap_or(str)
                .split("\n")
                .map(|num| -> u32 { num.parse().expect("Couldn't parse num!") })
                .sum()
        })
        .max()
        .unwrap();

    format!("{}", max)
}

pub fn part2(input: &String) -> String {
    "".to_owned()
}

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
