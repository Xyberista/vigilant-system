mod lib;

use std::fs;

use lib::*;

/// Runs the solution
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_file_path();
    let (clients, addable): (Vec<Client>, Ing) = parse_input(&path)?;

    // Debug
    println!("{}", clients.len());
    println!("{:?}", clients[0]);

    let pizza = find_pizza(&clients, addable);

    // output file
    fs::write("./output.txt", pizza.into_iter().collect::<Vec<_>>().join(" "))?;
    Ok(())
}