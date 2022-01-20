use std::{fs, io::{prelude::*, self}};

const INPUT_DIRECTORY: &str = "./input/";
const DEFAULT_INPUT: &str = "a_an_example.in.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    Ok(())
}

/// Gets input from default directory. 
/// Filename is passed through command line, and defaults to the constant [DEFAULT_INPUT].
fn get_input() -> Result<String, io::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let filename = if let Some(filename) = args.get(1) {
        filename
    } else {
        DEFAULT_INPUT
    };
    fs::read_to_string(format!("{}{}", INPUT_DIRECTORY, filename))
}