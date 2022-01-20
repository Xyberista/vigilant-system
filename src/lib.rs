use std::{
    collections::HashSet,
    fs,
    io::{self, prelude::*},
};

pub const INPUT_DIRECTORY: &str = "./input/";
pub const DEFAULT_INPUT: &str = "a_an_example.in.txt";

/// Ingredients type. (Ing short for Ingredients)
pub type Ing = HashSet<String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    likes: Ing,
    dislikes: Ing,
}

impl Client {
    fn new(likes: Ing, dislikes: Ing) -> Self {
        Self { likes, dislikes }
    }
}

/// Gets the path to the input file
///
/// Filename is passed through command line, and defaults to the constant [DEFAULT_INPUT].
/// The directory is provided by the constant [`INPUT_DIRECTORY`].
pub fn get_file_path() -> String {
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
/// This function takes a path as a paramter.
/// The file being read is either the one provided via command line
/// or the [`DEFAULT_INPUT`](default).
pub fn parse_input<S>(path: S) -> Result<(Vec<Client>, Ing), Box<dyn std::error::Error>>
where
    S: AsRef<std::path::Path>,
{
    let mut reader = io::BufReader::new(fs::File::open(&path)?);
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let number_of_clients: usize = buf.trim().parse()?;

    let mut clients: Vec<Client> = Vec::new();
    let mut addable: Ing = HashSet::new();
    for _ in 0..number_of_clients {
        // gets the ingredients liked by the client
        let mut likes = String::new();
        reader.read_line(&mut likes)?;
        let mut likes = likes.trim().split(' ');
        let _n_likes = likes.next().unwrap();
        let likes = likes.map(|s| s.to_string()).collect::<HashSet<String>>();
        addable.extend(likes.iter().cloned());

        // gets the ingredients disliked by the client
        let mut dislikes = String::new();
        reader.read_line(&mut dislikes)?;
        let mut dislikes = dislikes.trim().split(' ');
        let _n_dislikes = dislikes.next().unwrap();
        let dislikes = dislikes.map(|s| s.to_string()).collect::<HashSet<String>>();

        clients.push(Client::new(likes, dislikes));
    }
    Ok((clients, addable))
}

pub fn find_pizza(clients: &[Client], addable: Ing) -> Ing {
    todo!()
}

pub fn score(clients: &[Client], pizza: Ing) -> usize {
    let mut s = 0;
    for client in clients {
        if pizza.is_disjoint(&client.dislikes)
            && pizza.is_superset(&client.likes)
        {
            s += 1;
        }
    }
    s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score() {
        let path = "./input/a_an_example.in.txt";
        let (clients, addable): (Vec<Client>, Ing) = parse_input(&path).unwrap();
        assert_eq!(2, score(&clients, addable));
    }
}