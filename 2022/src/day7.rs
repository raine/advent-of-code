use std::{
    collections::HashMap,
    io::prelude::*,
    io::{BufReader, Lines},
};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Tree {
    Directory { children: HashMap<String, Tree> },
    File(u32),
}

impl Tree {
    fn traverse(&mut self, path: &[String]) -> &mut Tree {
        let mut cur = self;
        for p in path {
            if let Tree::Directory { children } = cur {
                cur = children.get_mut(p).expect("could not find path");
            }
        }

        cur
    }

    fn add(&mut self, path: &[String], ls_output: LsOutput) {
        let tree = self.traverse(path);
        if let Tree::Directory { children } = tree {
            match ls_output {
                LsOutput::File(name, size) => {
                    children.insert(name, Tree::File(size));
                }
                LsOutput::Directory(name) => {
                    children.insert(name, Tree::new_dir());
                }
            }
        }
    }

    fn get_size(&self) -> u32 {
        match self {
            Tree::Directory { children } => children
                .iter()
                .map(|(_dirname, tree)| tree.get_size())
                .sum(),
            Tree::File(size) => *size,
        }
    }

    fn get_directories(&self) -> Vec<Tree> {
        match self {
            Tree::Directory { children } => {
                children.iter().fold(vec![], |mut acc, (_path, tree)| {
                    if let Tree::Directory { children: _ } = tree {
                        acc.push(tree.clone());
                        acc.extend(tree.get_directories());
                    }
                    acc
                })
            }
            Tree::File(_) => vec![],
        }
    }

    fn new_dir() -> Tree {
        Tree::Directory {
            children: HashMap::new(),
        }
    }
}

impl<T: std::io::BufRead> From<Lines<T>> for Tree {
    fn from(lines: Lines<T>) -> Tree {
        let mut current_path: Vec<String> = vec![];
        let mut tree = Tree::new_dir();
        for line in lines {
            match parse_line(line.unwrap()) {
                ParsedLine::Command(cmd, arg) => match (cmd.as_ref(), arg.as_deref()) {
                    ("cd", Some("/")) => continue,
                    ("cd", Some(path)) => {
                        println!("cding to {path:?}");
                        match path {
                            ".." => {
                                current_path.pop();
                            }
                            _ => {
                                current_path.push(path.to_string());
                            }
                        }
                    }
                    ("ls", _) => println!("listing files"),
                    (_, _) => panic!("unexpected command"),
                },
                ParsedLine::LsOutput(ls_output) => {
                    println!("got file {ls_output:?}");
                    tree.add(&current_path, ls_output);
                }
            }
        }

        tree
    }
}

#[derive(Debug, PartialEq)]
enum LsOutput {
    File(String, u32),
    Directory(String),
}

#[derive(Debug, PartialEq)]
enum ParsedLine {
    Command(String, Option<String>),
    LsOutput(LsOutput),
}

fn day7<T>(reader: BufReader<T>) -> u32
where
    T: std::io::Read,
{
    let tree = Tree::from(reader.lines());
    tree.get_directories()
        .iter()
        .filter_map(|dir| match dir.get_size() {
            size if size <= 100_000 => Some(size),
            _ => None,
        })
        .sum()
}

const TOTAL_SPACE: u32 = 70_000_000;
const REQUIRED_SPACE: u32 = 30_000_000;

fn day7_part2<T>(reader: BufReader<T>) -> u32
where
    T: std::io::Read,
{
    let tree = Tree::from(reader.lines());
    let unused_space = TOTAL_SPACE - tree.get_size();
    let directories = tree.get_directories();

    directories
        .iter()
        .filter_map(|dir| match dir.get_size() {
            size if unused_space + size >= REQUIRED_SPACE => Some(size),
            _ => None,
        })
        .sorted()
        .next()
        .unwrap()
}

fn parse_line(line: String) -> ParsedLine {
    match line.chars().next() {
        Some('$') => parse_command(&line),
        Some(_) => parse_ls_output(&line),
        None => panic!("unexpected empty line"),
    }
}

fn parse_ls_output(line: &str) -> ParsedLine {
    let mut split = line.split(' ');
    let (first_col, name) = (split.next().unwrap(), split.next().unwrap());
    match first_col {
        "dir" => ParsedLine::LsOutput(LsOutput::Directory(name.to_string())),
        size => ParsedLine::LsOutput(LsOutput::File(
            name.to_string(),
            size.parse::<u32>().unwrap(),
        )),
    }
}

fn parse_command(line: &str) -> ParsedLine {
    let mut split = line.split(' ').skip(1);
    let (cmd, arg) = (split.next().unwrap(), split.next());
    ParsedLine::Command(cmd.to_string(), arg.map(|s| s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Cursor};

    #[test]
    fn test_day7_simple() {
        let input = "
$ cd /
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
7214296 k";

        let cursor = Cursor::new(input.trim_start_matches('\n'));
        let reader = BufReader::new(cursor);
        assert_eq!(day7(reader), 95437);
    }

    #[test]
    fn test_day7() {
        let input = File::open("./testdata/day7").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day7(reader), 1770595);
    }

    #[test]
    fn test_day7_part2() {
        let input = File::open("./testdata/day7").unwrap();
        let reader = BufReader::new(input);
        assert_eq!(day7_part2(reader), 2195372);
    }

    #[test]
    fn parse_command_test() {
        let input = "$ cd asdf";
        assert_eq!(
            parse_command(input),
            ParsedLine::Command("cd".to_string(), Some("asdf".to_string()))
        );

        let input = "$ ls";
        assert_eq!(
            parse_command(input),
            ParsedLine::Command("ls".to_string(), None)
        );
    }

    #[test]
    fn parse_ls_output_test() {
        let input = "dir a";
        assert_eq!(
            parse_ls_output(input),
            ParsedLine::LsOutput(LsOutput::Directory("a".to_string()))
        );
        let input = "123 a";
        assert_eq!(
            parse_ls_output(input),
            ParsedLine::LsOutput(LsOutput::File("a".to_string(), 123))
        );
    }

    #[test]
    fn tree_get_size_test() {
        let tree = Tree::Directory {
            children: HashMap::from([
                ("a".to_string(), Tree::File(2)),
                ("b".to_string(), Tree::File(2)),
                ("c".to_string(), Tree::File(2)),
                (
                    "d".to_string(),
                    Tree::Directory {
                        children: HashMap::from([
                            ("a".to_string(), Tree::File(1)),
                            ("b".to_string(), Tree::File(1)),
                        ]),
                    },
                ),
            ]),
        };

        assert_eq!(tree.get_size(), 8);
    }
}
