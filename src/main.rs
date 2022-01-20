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

/// Runs the solution
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = get_filename();
    let mut reader = io::BufReader::new(fs::File::open(&filename)?);
    let (clients, addable): (Vec<Client>, Ing) = parse_input(&mut reader)?;

    // Debug
    println!("{}", clients.len());
    println!("{:?}", clients[0]);

    let pizza = find_pizza(&clients, addable);

    // output file
    fs::write(filename.replace(".in", ".out"), pizza.into_iter().collect::<Vec<String>>().join(" "))?;
    Ok(())
}

/// Gets the filename of the input file.
///
/// Filename is passed through command line, and defaults to the constant [DEFAULT_INPUT].
fn get_filename() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    let filename = if let Some(filename) = args.get(1) {
        filename
    } else {
        DEFAULT_INPUT
    };
    format!("{}{}", INPUT_DIRECTORY, filename)
}

/// Parses input into a list of clients and hashset of addable ingredients.
///
/// This function takes a [`BufReader<File>`](`std::io::BufReader<std::fs::File>`) as a parameter.
/// The file being read is either the one provided via command line or the [`DEFAULT_INPUT`](default).
fn parse_input(
    reader: &mut io::BufReader<fs::File>,
) -> Result<(Vec<Client>, Ing), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let number_of_clients: usize = buf.trim().parse()?;

    let mut clients: Vec<Client> = Vec::new();
    let mut addable: Ing = HashSet::new();
    for _ in 0..number_of_clients {
        // gets the ingredients liked by the client
        let mut likes = String::new();
        reader.read_line(&mut likes)?;
        let mut likes = likes.split(' ');
        let _n_likes = likes.next().unwrap();
        let likes = likes.map(|s| s.to_string()).collect::<HashSet<String>>();
        addable.extend(likes.iter().cloned());

        // gets the ingredients disliked by the client
        let mut dislikes = String::new();
        reader.read_line(&mut dislikes)?;
        let mut dislikes = dislikes.split(' ');
        let _n_dislikes = dislikes.next().unwrap();
        let dislikes = dislikes.map(|s| s.to_string()).collect::<HashSet<String>>();

        clients.push(Client::new(likes, dislikes));
    }
    Ok((clients, addable))
}

fn find_pizza(clients: &[Client], addable: Ing) -> Ing {
    todo!()
}

fn score(clients: &[Client], pizza: Ing) -> usize {
    todo!()
}