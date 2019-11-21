use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guessing_game() {
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

fn takes_ownership(spatial_reference_name: String) { // spatial_reference_name comes into scope
    println!("The spatial reference is {}.", spatial_reference_name);
} // Here, spatial_reference_name goes out of scope and drop is called. The backing memory is freed.

fn makes_copy(wkid: i32) { // wkid comes into scope
    println!("The wkid is {}.", wkid);
} // Here, wkid goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it.
    // spatial_reference_name comes into scope
    let spatial_reference_name = String::from("WGS84");

    // spatial_reference_name is returned and moves out to the calling function
    spatial_reference_name
}

fn takes_and_gives_back(spatial_reference_name: String) -> String { // spatial_reference_name comes into scope
    // spatial_reference_name is returned and moves out to the calling function
    spatial_reference_name
}

fn understanding_ownership() {
    // spatial_reference_name comes into scope
    let spatial_reference_name = String::from("WGS84");

    // spatial_reference_name value is moved into the function
    // and so spatial_reference_name is no longer valid here
    takes_ownership(spatial_reference_name);

    // compile error!
    //println!("{}", spatial_reference_name);

    // wkid comes into scope
    let wkid = 4326;

    // wkid would move into function,
    // but i32 is Copy, so it is okay to still use wkid afterward.
    makes_copy(wkid);

    // no compile error
    println!("The wkid is {} again.", wkid);

    // gives_ownership moves its return value into spatial_reference_name
    let spatial_reference_name = gives_ownership();

    // spatial_reference_name_new comes into scope
    let spatial_reference_name_new = String::from("WGS84");

    // spatial_reference_name is moved into takes_and_gives_back,
    // which also moves its return value into spatial_reference_name_moved
    let spatial_reference_name_moved = takes_and_gives_back(spatial_reference_name_new);
}   // Here, spatial_reference_name_moved goes out of scope and is dropped.
    // spatial_reference_name_new goes out of scope but was moved, so nothing happens.
    // spatial_reference_name goies out of scope and is dropped.

fn use_borrowed(spatial_reference_name: &String) -> i32 { // spatial_reference_name is a reference to a String
    match spatial_reference_name.as_ref() {
        "WGS84" => 4326,
        _ => 0,
    }
} // Here, spatial_reference_name goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change_borrowed(spatial_reference_name: &mut String) -> i32 {
    match spatial_reference_name.as_ref() {
        "WGS84" => 4326,
        "" => { spatial_reference_name.push_str("WGS84"); 4326 }
        _ => 0,
    }
}



fn main() {
    // spatial_reference_name comes into scope
    let spatial_reference_name = String::from("WGS84");

    // pass a reference to spatial_reference_name into use_borrowed
    // a reference refers to the value of spatial_reference_name but does not own it.
    // because it does not own it, the value it points to will not be dropped when the reference goes out of scope.
    let wkid = use_borrowed(&spatial_reference_name);

    // wkid would move into function,
    // but i32 is Copy, so it is okay to still use wkid afterward.
    makes_copy(wkid);

    // mutable empty_name comes into scope
    // the mutable reference &mut empty_name is passed into change_borrowed
    let mut empty_name = String::from("");
    let wkid = change_borrowed(&mut empty_name);
    // here, empty_name was changed

    // wkid would move into function,
    // but i32 is Copy, so it is okay to still use wkid afterward.
    makes_copy(wkid);
}

