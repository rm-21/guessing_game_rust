use rand::Rng;
use std::io;

fn main() {
    println!("Guess the Number!");

    /*
    - thread_rng()
        --> gives the random number generator
        --> Local to the current thread of execution
        --> Tied to seed of the OS
    - gen_range()
        --> Defined by `Rng` trait
        --> how to define a range --> start..end --> start included, end excluded
        --> Or we could use start..=end --> 1..=100
    */
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    println!("Please provide a guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
