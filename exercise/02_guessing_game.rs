use std::io;


fn get_random_guess() -> u32{
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse().unwrap();
    guess
}

fn main(){
    // This will give you the Input of your comand line as a variable
    let guess: u32 = get_random_guess();
    println!("{}", guess);
}
