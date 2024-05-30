use std::io;

/// Ask the user to guess a number and returns the input as a String
fn input_guess() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    return guess.trim().to_string();
}

pub fn main() {
    let secret_number = 42;
    let mut guessed_numbers = Vec::new();

    for _ in 0..5 {
        println!("Enter a number: ");
        let guess: String = input_guess();
        println!("You guessed: {guess}");

        let guessed_number: i32 = guess.parse().unwrap();

        guessed_numbers.push(guessed_number);

        if guessed_number < secret_number {
            println!("The secret number is larger!");
        } else if guessed_number > secret_number {
            println!("The secret number is smaller!");
        } else {
            println!("Correct!");
            println!("{:?}", guessed_numbers);
            return;
        }
    }

    println!("You lose! The secret number was {}", secret_number);
    println!("{:?}", guessed_numbers);
}
