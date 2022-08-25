use rand::Rng;
use std::cmp::Ordering;
use std::io; // importing input/output from the standard library

fn main() {
    // fn indicates a new function
    println!("Guess the number!");

    // the gen_range method takes a range expression
    // of the form start..=end in as argument
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The screat number is: {secret_number}");

    // the loop keyword creates an infinite loop
    loop {
        println!("Please input your guess.");

        // in rust variables are immutable by default
        // we use the mut keyword to make it mutable
        // String::new() is a function that returns a
        // new instance of a string
        // new is an associated function; these are
        // functions that are implemented on a type eg String
        let mut guess = String::new();

        // if we didn't type use std::io at the beginning of the code base
        // we could have still used io::stdin() by type in std::io::stdin()
        io::stdin()
            .read_line(&mut guess) // the & inidicates that this argument is a reference
            // references are immutable by default, therefore,
            // there is a need to write the mut keyword after the & symbol
            .expect("Failed to read line"); // this is a method to handle potential failure

        // the trim method would eliminate any white space in the variable value;
        // we must do this to be able to convert the string to number
        // parse method on strings converts a string to another type
        // : u32 tells parse what type to convert to
        // and tells Rust what type should be stored in the variable
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // ignoring a non number guess
        // _ means catch all value 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        // cmp method compares two values and can be called on anything that can be compared
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
