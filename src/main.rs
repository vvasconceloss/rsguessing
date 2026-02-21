use std::io::stdin;

fn main() {
    println!("===================");
    println!("GUESSING THE NUMBER");
    println!("===================");

    let mut user_input = String::new();

    println!("Please input your guess: ");

    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the line");
}
