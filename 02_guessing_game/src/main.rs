use std::io;

/// Ask the user to guess a number and returns the input as a String
fn input_guess() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    return guess.trim().to_string();
}

fn main() {
    let guess: String = input_guess();
    println!("You guessed: {guess}");
    // Your code here
}
