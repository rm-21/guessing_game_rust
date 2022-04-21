use std::io;

fn main() {
    //PART 1
    println!("Guess the Number!");
    println!("Please provide a guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);

    //PART 2
}
