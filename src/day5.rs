use regex::Regex;

fn build_stacks(s: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>>;
    let lines = s.lines();

    // Crate locations are stored every (4n-3) locations

    // Total Strlen = (4n - 3) + 1
    // Total strlen = 4n-2
    // n = (len+2)/4
    // Get num of stacks. We don't ever see 2-digit nums
    let crate_locations = (lines.clone().last().unwrap().len() + 2) / 4;

    // Create outer vector of appropriate size
    stacks = Vec::with_capacity(crate_locations);

    for _ in 0..crate_locations {
        stacks.push(vec![]);
    }

    // Add crates to each stack
    for l in lines.rev().skip(1) {
        for i in (1..(crate_locations * 4 - 2)).step_by(4) {
            let c = l.chars().nth(i).unwrap();
            if c == ' ' {
                continue;
            }
            stacks
                .get_mut(((i + 3) / 4) - 1)
                .expect("Stack didn't exist!")
                .push(c);
        }
    }

    stacks
}

fn move_stack((times, from_stack, to_stack): (usize, usize, usize), s: &mut Vec<Vec<char>>) {
    for _ in 0..times {
        let c = s
            .get_mut(from_stack - 1)
            .unwrap_or_else(|| panic!("{} out of bounds!", from_stack))
            .pop()
            .unwrap_or_else(|| panic!("Tried to pop {} but it's empty!", from_stack));

        s.get_mut(to_stack - 1)
            .unwrap_or_else(|| panic!("{} out of bounds!", from_stack))
            .push(c);
    }
}

// Simple 3-junction push will do it quick and dirty
fn move_chunk((num_crates, from_stack, to_stack): (usize, usize, usize), s: &mut Vec<Vec<char>>) {
    let mut temp_v: Vec<char> = vec![];

    for _ in 0..num_crates {
        temp_v.push(
            s.get_mut(from_stack - 1)
                .unwrap_or_else(|| panic!("{} out of bounds|!", from_stack))
                .pop()
                .unwrap_or_else(|| panic!("Tried to pop {} but it's empty!", from_stack)),
        );
    }

    for _ in 0..num_crates {
        s.get_mut(to_stack - 1)
            .unwrap_or_else(|| panic!("{} out of bounds!", from_stack))
            .push(temp_v.pop().unwrap());
    }
}

fn get_stack_tops(s: &Vec<Vec<char>>) -> String {
    s.iter()
        .map(|v| v.iter().last().unwrap())
        .collect::<String>()
}

fn parse_moves_rg(s: &str) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"(\d+).*(\d+).*(\d+)").unwrap();
    let mut results: Vec<(usize, usize, usize)> = Vec::with_capacity(s.lines().count());

    for (_, [times, from, to]) in re.captures_iter(s).map(|c| c.extract()) {
        results.push((
            times.parse::<usize>().unwrap(),
            from.parse::<usize>().unwrap(),
            to.parse::<usize>().unwrap(),
        ))
    }

    results
}

fn parse_input(input: &String) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let (stacks_str, moves_str) = input
        .split_once("\n\n")
        .expect("Couldn't separate both parts!");

    let stacks = build_stacks(stacks_str);

    let moves = parse_moves_rg(moves_str);

    // _moves_str
    // .lines()
    // .map(|l| parse_move(l))
    // .collect::<Vec<(usize, usize, usize)>>();

    (stacks, moves)
}

pub fn part1(input: &String) -> String {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        move_stack(m, &mut stacks);
    }

    get_stack_tops(&stacks)
}

pub fn part2(input: &String) -> String {
    let (mut stacks, moves) = parse_input(input);

    for m in moves {
        move_chunk(m, &mut stacks);
    }

    get_stack_tops(&stacks)
}

#[cfg(test)]
mod tests {
    pub const TEST: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    pub const PART1_TEST_EXPECT: &str = "CMZ";
    pub const PART2_TEST_EXPECT: &str = "MCD";

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
