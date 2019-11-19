use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please guess the number.");    

    let secret_number = rand::thread_rng().gen_range(1, 10);

    loop {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to get the input!");

        // Shadowing the variable named number
        let number: u32 = match number.trim().parse() {
            Ok(integer) => integer,
            Err(_) => continue,
        };

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {}", number);
    }
}

