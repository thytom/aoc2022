use std::collections::{vec_deque, VecDeque};

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    ChangeDirectory(&'a str),
    Listing,
    File(&'a str, u64),
    Directory(&'a str),
    Unknown(&'a str),
}

fn lex_line(s: &str) -> Token {
    let mut cs = s.chars();

    match cs.next().unwrap() {
        '$' => {
            _ = cs.next();
            match cs.next().unwrap() {
                'l' => Token::Listing,
                'c' => Token::ChangeDirectory(&s[5..]),
                _ => Token::Unknown(s),
            }
        }

        'd' => Token::Directory(&s[4..]),

        _ => {
            let (size_str, name) = s.split_once(" ").unwrap();
            Token::File(name, size_str.parse::<u64>().unwrap())
        }
    }
}

#[derive(Debug)]
enum Node {
    Directory(String, VecDeque<Node>),
    File(u64),
}

fn parse_directory(v: &mut VecDeque<Token>, contents: &mut VecDeque<Node>) {
    // First thing we see upon cd should be either cd .. or ls

    let first_token = v.pop_front();

    // Validate the first token
    match first_token {
        Some(Token::Listing) => {}

        Some(x) => {
            panic!("Can't have {:#?} after dir!", x);
        }

        None => {
            return;
        }
    }

    loop {
        // Get the next token
        let next_token = v.pop_front();

        match next_token {
            Some(Token::File(_, size)) => {
                contents.push_back(Node::File(size));
            }

            Some(Token::ChangeDirectory(s)) => {
                // Are we going up?
                if s == ".." {
                    // End this listing
                    break;
                } else {
                    // TODO Subdirs
                    //todo!()
                    for elem in &mut *contents {
                        match elem {
                            Node::Directory(id, lv) => {
                                if id == s {
                                    parse_directory(v, lv);
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }

            Some(Token::Directory(id)) => {
                contents.push_back(Node::Directory(id.to_owned(), VecDeque::new()));
            }

            Some(Token::Listing) => {
                panic!("Cannot have nested listings!")
            }

            Some(Token::Unknown(s)) => {
                panic!("Unknown token {}!", s)
            }

            None => {
                break;
            }
        }
    }
}

fn parse_token_list(v: &mut VecDeque<Token>) -> Node {
    match v.pop_front().unwrap() {
        Token::ChangeDirectory(s) => {
            let mut listing: VecDeque<Node> = VecDeque::new();
            parse_directory(v, &mut listing);
            Node::Directory(s.to_owned(), listing)
        }
        _ => todo!(),
    }
}

fn size_of(n: &Node) -> u64 {
    match n {
        Node::Directory(_, v) => v.iter().map(|dir_node| size_of(dir_node)).sum(),
        Node::File(x) => *x,
    }
}

fn directory_size_listings(n: &Node, v: &mut Vec<u64>) {
    match n {
        Node::Directory(_, elems) => {
            // First, get the size of this directory and push it
            let this_size = size_of(n);

            v.push(this_size);

            // Now, for all child directories, do the same.
            elems.iter().for_each(|e| match e {
                Node::Directory(_, _) => directory_size_listings(e, v),
                Node::File(_) => (),
            })
        }
        Node::File(_) => (), // We shouldn't do anything about files
    }
}

pub fn part1(input: &String) -> String {
    let mut tokens = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .map(|l| lex_line(l))
        .collect::<VecDeque<Token>>();

    let fs = parse_token_list(&mut tokens);

    let mut sizes: Vec<u64> = Vec::new();

    directory_size_listings(&fs, &mut sizes);

    sizes
        .iter()
        .filter(|x| **x < 100000)
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &String) -> String {
    let mut tokens = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .map(|l| lex_line(l))
        .collect::<VecDeque<Token>>();

    let fs = parse_token_list(&mut tokens);

    let mut sizes: Vec<u64> = Vec::new();

    directory_size_listings(&fs, &mut sizes);

    // get the size of the outer dir
    let total_size = size_of(&fs);

    let required_space = 30000000 - (70000000 - total_size);

    // Find all dirs greater than 70 000 000 - total_size
    sizes
        .iter()
        .filter(|e| **e > required_space)
        .min()
        .expect("No dirs big enough!")
        .to_string()
}

#[cfg(test)]
mod tests {
    pub const TEST: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    pub const PART1_TEST_EXPECT: &str = "95437";
    pub const PART2_TEST_EXPECT: &str = "24933642";

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
