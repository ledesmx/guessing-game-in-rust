use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // secret_number is a i32 by default
    let secret_number = rand::thread_rng().gen_range(1..=1000);
    println!("The secret number is {secret_number}");

    println!("Guess the number between 1 and 1000");

    println!("Please input your guess.");
    let mut tries = 0;
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // trim() removes any whitespace at the beggining and the end of the line
        // parse() converts to another type, we need to specify with : type
        // parse() returns a Result type that can be handle by the match experssion
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        tries += 1;

        println!("Your guess is {}", guess);

        // Ordering is a enum, and has the variants Less, Greater & Equal

        // cmp() method compares two values and can be called on anything
        // that can be compared. It takes a reference and returns a variant
        // of the Ordering enum

        // the match expression is made up of arms, each arm consits of a
        // pattern to match, and the code that should be run in it
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("Your tries: {tries}");
    }

    println!("You finally did it at the {tries} attempt");
}
