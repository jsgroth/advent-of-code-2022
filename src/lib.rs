use std::{env, fs, io};
use std::path::Path;

// Read the contents of the file specified by the first command line argument, one string per line
pub fn read_input() -> io::Result<Vec<String>> {
    let mut args = env::args();

    // args[0] is executable path
    args.next();

    let input_path = args.next().ok_or(
        io::Error::new(io::ErrorKind::NotFound, "missing input file arg")
    )?;

    let raw_contents = fs::read_to_string(Path::new(&input_path))?;

    let mut lines: Vec<String> = raw_contents.split('\n')
        .map(|line| String::from(line))
        .collect();

    // Remove empty string from end of list
    if raw_contents.chars().last() == Some('\n') {
        lines.remove(lines.len() - 1);
    }

    Ok(lines)
}