use ahash::AHashSet;
use rayon::prelude::*;
use std::{
    fs,
    io::{self, prelude::*},
};

pub const INPUT_DIRECTORY: &str = "./input/";
pub const DEFAULT_INPUT: &str = "a_an_example.in.txt";

/// Ingredients type. (Ing short for Ingredients)
pub type Ing = AHashSet<String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    pub likes: Ing,
    pub dislikes: Ing,
}

impl Client {
    fn new(likes: Ing, dislikes: Ing) -> Self {
        Self { likes, dislikes }
    }
}

/// Gets the path to the input file
///
/// Filename is passed through command line, and defaults to the constant [`DEFAULT_INPUT`].
/// Filename can also be passed as a parameter, defaulting to `None`.
/// The directory is provided by the constant [`INPUT_DIRECTORY`].
pub fn get_file_path(filename: Option<&str>) -> String {
    let args = std::env::args().collect::<Vec<String>>();
    let filename = if let Some(filename) = filename {
        filename
    } else if let Some(filename) = args.get(1) {
        filename
    } else {
        DEFAULT_INPUT
    };
    format!("{}{}", INPUT_DIRECTORY, filename)
}

/// Parses input into a list of clients and AHashSet of addable ingredients and removeable ingredients.
///
/// This function takes a path as a paramter.
/// The file being read is either the one provided via command line
/// or the [default](`DEFAULT_INPUT`).
pub fn parse_input<S>(path: S) -> Result<(Vec<Client>, Ing, Ing), Box<dyn std::error::Error>>
where
    S: AsRef<std::path::Path>,
{
    let mut reader = io::BufReader::new(fs::File::open(&path)?);
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    let number_of_clients: usize = buf.trim().parse()?;

    let mut clients: Vec<Client> = Vec::new();
    let mut addable: Ing = AHashSet::new();
    let mut removeable: Ing = AHashSet::new();
    for _ in 0..number_of_clients {
        // gets the ingredients liked by the client
        let mut likes = String::new();
        reader.read_line(&mut likes)?;
        let mut likes = likes.trim().split(' ');
        let _n_likes = likes.next().unwrap();
        let likes = likes.map(|s| s.to_string()).collect::<AHashSet<String>>();
        addable.extend(likes.iter().cloned());

        // gets the ingredients disliked by the client
        let mut dislikes = String::new();
        reader.read_line(&mut dislikes)?;
        let mut dislikes = dislikes.trim().split(' ');
        let _n_dislikes = dislikes.next().unwrap();
        let dislikes = dislikes
            .map(|s| s.to_string())
            .collect::<AHashSet<String>>();
        removeable.extend(dislikes.iter().cloned());

        clients.push(Client::new(likes, dislikes));
    }
    Ok((clients, addable, removeable))
}

/// Finds the best pizza with the given algorithm
///
/// The algorithms are specified in the module [`algorithms`]
pub fn find_pizza<F>(clients: &[Client], addable: &Ing, removeable: &Ing, algorithm: F) -> Ing
where
    F: Fn(&[Client], &Ing, &Ing) -> Ing,
{
    algorithm(clients, addable, removeable)
}

/// Gets the score of the current pizza
///
/// This returns the number of clients that will be satisfied by the pizza.
/// Conditions for being satisfied:
/// - all of the liked toppings are on the pizza
/// - all of the disliked toppings are not on the pizza
pub fn score(clients: &[Client], pizza: &Ing) -> usize {
    clients
        .par_iter()
        .fold(
            || 0_usize,
            |a, c| {
                if pizza.is_disjoint(&c.dislikes) && pizza.is_superset(&c.likes) {
                    a + 1
                } else {
                    a
                }
            },
        )
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score() {
        let path = "./input/a_an_example.in.txt";
        let (clients, addable, _removeable): (Vec<Client>, Ing, Ing) = parse_input(&path).unwrap();
        assert_eq!(2, score(&clients, &addable));
    }
}
