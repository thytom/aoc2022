const SCORE_MATRIX: [[u32; 3]; 3] = [
    [4, 8, 3], // A
    [1, 5, 9], // B
    [7, 2, 6], // C
];

const SCORE_MATRIX_2: [[u32; 3]; 3] = [
    [3, 4, 8], // A
    [1, 5, 9], // B
    [2, 6, 7], // C
];

fn lines_to_scores(matrix: [[u32; 3]; 3], input: &String) -> Vec<u32> {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .map(|str: &str| -> u32 {
            let coord = (
                str.chars().nth(0).unwrap() as u8 - 'A' as u8,
                str.chars().nth(2).unwrap() as u8 - 'X' as u8,
            );
            matrix[coord.0 as usize][coord.1 as usize]
        })
        .collect::<Vec<u32>>()
}

pub fn part1(input: &String) -> String {
    format!(
        "{}",
        lines_to_scores(SCORE_MATRIX, input).iter().sum::<u32>()
    )
}

pub fn part2(input: &String) -> String {
    format!(
        "{}",
        lines_to_scores(SCORE_MATRIX_2, input).iter().sum::<u32>()
    )
}

#[cfg(test)]
mod tests {
    pub const TEST: &str = r#"A Y
B X
C Z"#;

    pub const PART1_TEST_EXPECT: &str = "15";
    pub const PART2_TEST_EXPECT: &str = "12";

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
