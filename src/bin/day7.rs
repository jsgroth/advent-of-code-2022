//! Day 7: No Space Left On Device
//! https://adventofcode.com/2022/day/7

use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;
use std::rc::{Rc, Weak};

struct File<'a> {
    _name: &'a str,
    size: u32,
}

impl<'a> File<'a> {
    fn new(name: &str, size: u32) -> File {
        File { _name: name, size }
    }
}

struct Directory<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    subdirectories: HashMap<&'a str, Rc<RefCell<Directory<'a>>>>,
    parent_directory: Option<Weak<RefCell<Directory<'a>>>>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str, parent_directory: Option<Rc<RefCell<Directory<'a>>>>) -> Directory<'a> {
        let parent_directory = parent_directory.map(|d| Rc::downgrade(&d));
        Directory {
            name,
            files: Vec::new(),
            subdirectories: HashMap::new(),
            parent_directory,
        }
    }

    fn add_file(&mut self, file: File<'a>) {
        self.files.push(file);
    }

    fn add_subdirectory(&mut self, subdirectory: Directory<'a>) -> Option<Rc<RefCell<Directory<'a>>>> {
        self.subdirectories.insert(subdirectory.name, Rc::new(RefCell::new(subdirectory)))
    }

    fn get_subdirectory(&self, name: &str) -> Option<Rc<RefCell<Directory<'a>>>> {
        match self.subdirectories.get(name) {
            Some(dir) => Some(Rc::clone(dir)),
            None => None,
        }
    }

    // Fold over every node in the directory tree and return (total size, fold result)
    fn total_size_traverse<T, F>(&self, initial_state: T, f: F) -> (u32, T)
    where
        F: Copy + Fn(T, u32) -> T,
    {
        let files_size: u32 = self.files.iter().map(|file| file.size).sum();

        let mut directories_size = 0;
        let mut state = initial_state;
        for d in self.subdirectories.values() {
            let (total_size, sub_dir_state) = d.borrow().total_size_traverse(state, f);

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

    let (root_dir_total_size, solution1) = root_dir.borrow().total_size_traverse(0, |acc, total_size| {
        if total_size <= PART_1_MAX_DIRECTORY_SIZE {
            acc + total_size
        } else {
            acc
        }
    });

    let target_space = PART_2_DISK_SIZE - PART_2_TARGET_FREE_SPACE;
    let (_, solution2) = root_dir.borrow().total_size_traverse(u32::MAX, |acc, total_size| {
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

    let root_dir = Rc::new(RefCell::new(Directory::new("/", None)));

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
                handle_ls_command(&current_dir, &ls_output);
            }
            _ => panic!("only supported commands are cd and ls; command={command}")
        }
    }

    root_dir
}

fn handle_cd_command<'a>(current_dir: &Rc<RefCell<Directory<'a>>>, dir_name: &str) -> Rc<RefCell<Directory<'a>>> {
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

        result.push(*line);
        iter.next();
    }

    result
}

fn handle_ls_command<'a>(current_dir: &Rc<RefCell<Directory<'a>>>, ls_output: &[&'a str]) {
    for line in ls_output {
        let (size, name) = line.split_once(' ').expect("line in ls output should have one space");
        if size == "dir" {
            let directory = Directory::new(name, Some(Rc::clone(current_dir)));
            if let Some(_) = current_dir.borrow_mut().add_subdirectory(directory) {
                panic!("directory {} already exists in current directory {}", name, current_dir.borrow().name);
            }
        } else {
            let size: u32 = size.parse().expect("size should be an integer");
            current_dir.borrow_mut().add_file(File::new(name, size));
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
7214296 k";

    #[test]
    fn test_sample_input() {
        let (solution1, solution2) = solve(SAMPLE_INPUT);
        assert_eq!(95437, solution1);
        assert_eq!(24933642, solution2);
    }
}