use std::io;

fn main() {
    println!("Guess the Number!");
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
