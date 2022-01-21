#![allow(dead_code)]

use std::fs;

use lib::*;

const INPUT_FILES: [&str; 5] = [
    "a_an_example.in.txt",
    "b_basic.in.txt",
    "c_coarse.in.txt",
    "d_difficult.in.txt",
    "e_elaborate.in.txt",
];

/// Runs the solution
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_all()
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_file_path(None);
    let (clients, addable): (Vec<Client>, Ing) = parse_input(&path)?;

    // Debug
    println!("{}", clients.len());
    println!("{:?}", clients[0]);

    let pizza = find_pizza(&clients, &addable, algorithms::all);
    let len = pizza.len();

    // output file
    fs::write(
        "./output.txt",
        format!("{len} {}", pizza.into_iter().collect::<Vec<_>>().join(" ")),
    )?;
    Ok(())
}

fn run_all() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(()) = fs::create_dir("./output/") {}
    for file in INPUT_FILES {
        let path = get_file_path(Some(file));
        let (clients, addable): (Vec<Client>, Ing) = parse_input(&path)?;

        let pizza = find_pizza(&clients, &addable, algorithms::one_client);
        let len = pizza.len();

        println!("Score: {}", score(&clients, &pizza));

        // output file
        fs::write(
            format!("./output/{}", file.replace(".in", ".out")),
            format!("{len} {}", pizza.into_iter().collect::<Vec<_>>().join(" ")),
        )?;
    }
    Ok(())
}
