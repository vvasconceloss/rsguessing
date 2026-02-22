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

    let user_guess: u32 = user_input
        .trim()
        .parse::<u32>()
        .expect("Failed to convert to u32");

    match user_guess {
        1..=5 => {}
        _ => println!("The number must be between 1 and 5"),
    }
}
