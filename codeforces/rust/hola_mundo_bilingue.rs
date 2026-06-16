use std::io::{self, Write};
use std::str::FromStr;

fn get_input<T: FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input, try again."),
        }
    }
}

fn main() {
    let age: u32 = get_input("How old are you? ");
    let name: String = get_input("What's your name? ");

    println!("{} is {} years old", name, age);
}