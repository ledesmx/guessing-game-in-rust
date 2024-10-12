use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=1000);
    println!("The secret number is {secret_number}");

    println!("Guess the number between 1 and 1000");

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("Your guess is {}", guess);
}
