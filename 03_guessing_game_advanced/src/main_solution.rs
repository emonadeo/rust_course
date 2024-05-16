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

    loop {
        let guess: String = input_guess();
        println!("You guessed: {guess}");

        // you can specify the type `i32` on the variable...
        let guessed_number: i32 = guess.parse().unwrap();
        // or on the parse method using the so called „turbofish“ syntax...
        let guessed_number = guess.parse::<i32>().unwrap();

        // append `guessed_number` to `guessed_numbers`
        guessed_numbers.push(guessed_number);

        if guessed_number < secret_number {
            println!("The secret number is larger!");
        } else if guessed_number > secret_number {
            println!("The secret number is smaller!");
        } else {
            break;
        }
    }

    println!("Correct!");
    println!("{:?}", guessed_numbers);
}
