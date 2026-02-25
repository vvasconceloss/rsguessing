# rsguessing

Minimal number guessing game written in Rust.

## Features

- Random number generation using the `rand` crate
- Input validation and error handling
- Higher / Lower hints on each guess
- Win detection with attempt counter

## Usage

```bash
cargo run
```

## Gameplay

```
Guess the number!
Please input your guess: 4
Too high!
Please input your guess: 2
Too low!
Please input your guess: 3
You win! You guessed it in 3 attempts.
```

## Dependencies

- [`rand`](https://crates.io/crates/rand) — random number generation
