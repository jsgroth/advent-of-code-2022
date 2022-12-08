//! Day 7: No Space Left On Device
//! https://adventofcode.com/2022/day/7

use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;
use std::rc::{Rc, Weak};

struct Directory {
    file_sizes: Vec<u32>,
    subdirectories: HashMap<String, Rc<RefCell<Directory>>>,
    parent_directory: Option<Weak<RefCell<Directory>>>,
}

impl Directory {
    fn new(parent_directory: Option<Weak<RefCell<Directory>>>) -> Directory {
        Directory {
            file_sizes: Vec::new(),
            subdirectories: HashMap::new(),
            parent_directory,
        }
    }

    fn add_file_size(&mut self, file_size: u32) {
        self.file_sizes.push(file_size);
    }

    fn add_subdirectory(&mut self, name: String, subdirectory: Directory) -> Option<Rc<RefCell<Directory>>> {
        self.subdirectories.insert(name, Rc::new(RefCell::new(subdirectory)))
    }

    fn get_subdirectory(&self, name: &str) -> Option<Rc<RefCell<Directory>>> {
        match self.subdirectories.get(name) {
            Some(dir) => Some(Rc::clone(dir)),
            None => None,
        }
    }

    // Fold over the total size of every node in the directory tree and return (total size, fold result)
    fn total_size_fold<T, F>(&self, initial_state: T, f: F) -> (u32, T)
    where
        F: Copy + Fn(T, u32) -> T,
    {
        let files_size: u32 = self.file_sizes.iter().sum();

        let mut directories_size = 0;
        let mut state = initial_state;
        for d in self.subdirectories.values() {
            let (total_size, sub_dir_state) = d.borrow().total_size_fold(state, f);

            directories_size += total_size;
            state = sub_dir_state;
        }

        let total_size = files_size + directories_size;
        state = f(state, total_size);

        (total_size, state)
    }
}

const PART_1_MAX_DIRECTORY_SIZE: u32 = 100000;

const PART_2_DISK_SIZE: u32 = 70000000;
const PART_2_TARGET_FREE_SPACE: u32 = 30000000;

fn solve(input: &str) -> (u32, u32) {
    let root_dir = parse_input(input);

    let (root_dir_total_size, solution1) = root_dir.borrow().total_size_fold(0, |acc, total_size| {
        if total_size <= PART_1_MAX_DIRECTORY_SIZE {
            acc + total_size
        } else {
            acc
        }
    });

    let target_space = PART_2_DISK_SIZE - PART_2_TARGET_FREE_SPACE;
    let (_, solution2) = root_dir.borrow().total_size_fold(u32::MAX, |acc, total_size| {
        let new_space = root_dir_total_size - total_size;
        if new_space <= target_space && total_size < acc {
            total_size
        } else {
            acc
        }
    });

    (solution1, solution2)
}

fn parse_input(input: &str) -> Rc<RefCell<Directory>> {
    assert_eq!(input.lines().next(), Some("$ cd /"));

    let root_dir = Rc::new(RefCell::new(Directory::new(None)));

    let mut current_dir = Rc::clone(&root_dir);
    let mut lines = input.lines().skip(1).peekable();
    while let Some(line) = lines.next() {
        assert_eq!(line.chars().next(), Some('$'));

        let mut split = line.split_whitespace().skip(1);
        let command = split.next().expect("expecting cd or ls command after $");
        match command {
            "cd" => {
                let dir_name = split.next().expect("should be a directory name after cd command");
                current_dir = handle_cd_command(&current_dir, dir_name);
            }
            "ls" => {
                let ls_output = collect_ls_output(&mut lines);
                handle_ls_command(&mut current_dir, &ls_output);
            }
            _ => panic!("only supported commands are cd and ls; command={command}")
        }
    }

    root_dir
}

fn handle_cd_command(current_dir: &Rc<RefCell<Directory>>, dir_name: &str) -> Rc<RefCell<Directory>> {
    if dir_name == ".." {
        current_dir.borrow().parent_directory.as_ref().expect("should not be in root directory")
            .upgrade().expect("parent directory should not have been deallocated")
    } else {
        current_dir.borrow().get_subdirectory(dir_name).expect("current dir should have the given subdirectory")
    }
}

fn collect_ls_output<'a, I>(iter: &mut Peekable<I>) -> Vec<&'a str>
where
    I: Iterator<Item = &'a str>,
{
    let mut result: Vec<&str> = Vec::new();

    while let Some(line) = iter.peek() {
        if line.chars().next() == Some('$') {
            break;
        }

        result.push(iter.next().unwrap());
    }

    result
}

fn handle_ls_command(current_dir: &mut Rc<RefCell<Directory>>, ls_output: &[&str]) {
    for line in ls_output {
        let (size, name) = line.split_once(' ').expect("line in ls output should have one space");
        if size == "dir" {
            let directory = Directory::new(Some(Rc::downgrade(current_dir)));
            if let Some(_) = current_dir.borrow_mut().add_subdirectory(String::from(name), directory) {
                panic!("directory {} already exists in current directory", name);
            }
        } else {
            let size: u32 = size.parse().expect("size should be an integer");
            current_dir.borrow_mut().add_file_size(size);
        }
    }
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let (solution1, solution2) = solve(&input);
    println!("{solution1}");
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
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
7214296 k
";

    #[test]
    fn test_sample_input() {
        let (solution1, solution2) = solve(SAMPLE_INPUT);
        assert_eq!(95437, solution1);
        assert_eq!(24933642, solution2);
    }
}