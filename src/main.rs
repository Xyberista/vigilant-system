use std::{fs, io::{prelude::*, self}};

const INPUT_DIRECTORY: &str = "./input/";
const DEFAULT_INPUT: &str = "a_an_example.in.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = io::BufReader::new(get_filename()?);
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let clients: usize = buf.trim().parse()?;
    Ok(())
}

/// Filename is passed through command line, and defaults to the constant [DEFAULT_INPUT].
fn get_filename() -> Result<fs::File, io::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let filename = if let Some(filename) = args.get(1) {
        filename
    } else {
        DEFAULT_INPUT
    };
    fs::File::open(format!("{}{}", INPUT_DIRECTORY, filename))
}