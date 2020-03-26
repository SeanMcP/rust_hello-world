use std::io;
use std::process;

fn main() {
    println!("What is your name?");

    let mut name = String::new();

    io::stdin().read_line(&mut name).unwrap();

    match name.trim().parse() {
        Ok(value) => {
            name = value;
        },
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
    }

    println!("Hello, {}!", name);
}
