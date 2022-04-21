use std::io;

fn main() {
    println!("Guess the Number!");
    println!("Please input your guess: ");

    /*
    - let --> Defines a new variable
    - New variable `guess` --> By default immutable
    - mut --> Makes a variable mutable
    - new() --> Function that instantiates String type
    */
    let mut guess = String::new();

    /*
    - io::stdin() is the same as std::io::stdin()
    - io::stdin() is an instance of std::io::Stdin --> reader = std::io::Stdin:new()
    */
    io::stdin()
        /*
        - read_line() --> method to get input from the user
                  --> appends the input to a string variable without overwriting the contents
                  --> to do that, we pass the string variable (`guess`) as an input to the method
                  --> the **argument** needs to be mutable. We have defined `guess` as mutable. Necessary to write content in the variable
                  --> Returns io::Result
                      --> encode error handling information
                      --> `Ok`: Operation was successfull
                      --> `Err`: Operation failed. Contains info on why it failed
        - & --> indicated that the argument is a reference | basically points to the memory location of a variable
        - References are immutable by default. Therefore, `&guess` is an immutable reference, `&mut guess` is a mutable reference
        - read_line
        */
        .read_line(&mut guess)
        /*
        io:Result has the `expect` method
        */
        .expect("Failed to read line");

    // io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
