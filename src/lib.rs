use std::{fs, io};
use std::path::Path;

pub fn read_input(day: u32) -> io::Result<Vec<String>> {
    let path_str = format!("input/input{day}.txt");

    let raw_contents = fs::read_to_string(Path::new(&path_str))?;

    let mut lines: Vec<String> = raw_contents.split('\n')
        .map(|line| String::from(line))
        .collect();

    // Remove empty string from end of list
    if raw_contents.chars().last() == Some('\n') {
        lines.remove(lines.len() - 1);
    }

    Ok(lines)
}