use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // print this for now to ensure that this works
    // println!("The secret number is: {secret_number}");
    // uncomment the above for debugging

    // setup the logic to loop until the right guess is selected.
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust allows us to shadow the previous value of guess and this is the 
        // method typically used to convert a value from one type to another
        // guess.trim is called on the original string guess, then parse is used
        // to convert it to u32 as set by guess: u32, and expect is for errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // now have the secret_number, and the users guess so compare the two 
        // to tell the user if the guess was higher, lower, or correct

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), 
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // break out of the loop because the user guessed the number
                break;
            }
        }
    }
    // Create the logic for picking a random number between 1 and 100 to guess
    // first import the rand crate
    // The project that we are making here is called a binary crate, the rand
    // crate is a library crate which contains code that is intended to be
    // used in other programs and cannot be executed on its own. To use the 
    // rand crate Cargo.toml needs to be updated to list rand as a dependency

}
