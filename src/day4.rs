fn parse_pair(p: &str) -> (u32, u32) {
    let (l, r) = p.split_once("-").expect("Missing - !");

    (l.parse().unwrap(), r.parse().unwrap())
}

fn engulfing((a, b): ((u32, u32), (u32, u32))) -> u32 {
    let (al, ar) = a;
    let (bl, br) = b;

    if (al <= bl && ar >= br) || (al >= bl && ar <= br) {
        1
    } else {
        0
    }
}

pub fn part1(input: &String) -> String {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .map(|l| -> u32 {
            let (e1, e2) = l.split_once(',').expect("Mising comma!");

            engulfing((parse_pair(e1), parse_pair(e2)))
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &String) -> String {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .map(|l| -> u32 {
            let (e1, e2) = l.split_once(',').expect("Mising comma!");
            let a @ ((al, ar), (bl, br)) = (parse_pair(e1), parse_pair(e2));

            if (al >= bl && al <= br) || (ar <= br && ar >= bl) {
                1
            } else {
                engulfing(a)
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    pub const TEST: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    pub const PART1_TEST_EXPECT: &str = "2";
    pub const PART2_TEST_EXPECT: &str = "4";

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
