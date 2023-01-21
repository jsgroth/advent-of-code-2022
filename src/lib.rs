use std::path::Path;
use std::{env, fs, io};

// Read the contents of the file specified by the first command line argument
pub fn read_input() -> io::Result<String> {
    let mut args = env::args();

    // args[0] is executable path
    args.next();

    let input_path = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "missing input file arg"))?;

    fs::read_to_string(Path::new(&input_path))
}
