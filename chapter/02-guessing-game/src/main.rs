// This is due to bizarre corner case because Rust has a "hidden" rand create.
//
// https://stackoverflow.com/questions/30735490/unresolved-name-randthread-rng
extern crate rand;

// Trait for generating random numbers.
use rand::Rng;

// Traits, helpers, and type definitions for core I/O functionality.
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    // generate a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess, from 1 to 100.");

        // let defines a variable
        // mut means it is a mutable, otherwise immutable is the default
        // String is a growable, UTF-8 encoded bit of text
        // String::new is an "associated function" new on the type String
        let mut guess = String::new();

        // like variables, references are immutable by default, so we add &mut
        // result is an enum, and it's possible to be 'Ok' or 'Err'
        // while expect is not required, it would trigger a compiler warning
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // rust lets you redefine (shadow) local variables, neat
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid guess: {guess}");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // pattern matching is awesome.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
