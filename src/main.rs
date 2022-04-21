use rand::Rng;
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

    // Print the user input
    println!("You guessed {}", guess);
}
