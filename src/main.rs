use std::{
    collections::HashSet,
    fs,
    io::{self, prelude::*},
};

const INPUT_DIRECTORY: &str = "./input/";
const DEFAULT_INPUT: &str = "a_an_example.in.txt";

/// Ingredients type. (Ing short for Ingredients)
type Ing = HashSet<String>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Client {
    likes: Ing,
    dislikes: Ing,
}

impl Client {
    fn new(likes: Ing, dislikes: Ing) -> Self {
        Self { likes, dislikes }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = io::BufReader::new(get_filename()?);
    let clients: Vec<Client> = parse_input(&mut reader)?;

    // Debug
    println!("{}", clients.len());
    println!("{:?}", clients[0]);
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

fn parse_input(
    reader: &mut io::BufReader<fs::File>,
) -> Result<Vec<Client>, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let number_of_clients: usize = buf.trim().parse()?;
    let mut clients: Vec<Client> = Vec::new();
    for _ in 0..number_of_clients {
        let mut likes = String::new();
        reader.read_line(&mut likes)?;
        let mut likes = likes.split(' ');
        let _n_likes = likes.next().unwrap();
        let likes = likes.map(|s| s.to_string()).collect::<HashSet<String>>();

        let mut dislikes = String::new();
        reader.read_line(&mut dislikes)?;
        let mut dislikes = dislikes.split(' ');
        let _n_dislikes = dislikes.next().unwrap();
        let dislikes = dislikes.map(|s| s.to_string()).collect::<HashSet<String>>();

        clients.push(Client::new(likes, dislikes));
    }
    Ok(clients)
}
