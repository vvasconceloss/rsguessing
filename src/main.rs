use std::io::stdin;

fn main() {
    println!("===================");
    println!("GUESSING THE NUMBER");
    println!("===================");

    let mut attempts: u32 = 1;
    let secret_number: u32 = rand::random_range(1..=5);

    loop {
        let mut user_input = String::new();

        println!("Please input your guess: ");

        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the line");

        let user_guess: u32 = user_input
            .to_string()
            .trim()
            .parse()
            .expect("Failed to convert to u32");

        match user_guess {
            1..=5 => {
                if user_guess != secret_number {
                    if secret_number < user_guess {
                        println!("Too high!");
                    } else {
                        println!("Too low!");
                    }

                    attempts = attempts + 1;
                }

                if user_guess == secret_number {
                    println!("You win! You guessed it in {} attempts", attempts);
                    break;
                }
            }
            _ => println!("The number must be between 1 and 5"),
        }
    }
}
