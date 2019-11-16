use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please guess the number.");

    let mut number = String::new();

    let secret_number = rand::thread_rng().gen_range(1, 10);

    io::stdin().read_line(&mut number).expect("Failed to get the input!");

    println!("You guessed: {} and it was: {}", number, secret_number);
}

