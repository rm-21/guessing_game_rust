use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    println!("Please provide a guess!");

    // Define the variable
    let mut guess = String::new();

    // Read the user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    /*
    - Rust allows to shadow already defined variable
    - guess.trim().parse():
        --> here `guess` is the original guess variable in string format
        --> trim --> removes whitespaces from beg. and end
        --> parse --> parses the string into some kind of number. We need explicitly define the number type using `let guess u32`
    */
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // Print the user input
    println!("You guessed {}", guess);


    /*
    - String has a method `cmp` that compares two values
    - Ordering:
        --> is an enum
        --> Has 3 returns --> Less, Greater, Equal
    - match:
        --> Made up of **arms**
        --> Arm contains a pattern to match against, and code to run if a match is found
        --> Arm is given by `Odering::Less/Greater/Equal`
        --> Code to run is given by `=>`
    */
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal ==> println!("You win!"),
    }
}
