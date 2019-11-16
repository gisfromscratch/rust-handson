use std::io;


fn main() {
    println!("Guess the number!");
    println!("Please guess the number.");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Failed to get the input!");

    println!("You guessed: {}", number);
}

