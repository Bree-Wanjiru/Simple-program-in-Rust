use std::io;
use rand::Rng;

fn main() {
    println!("🎯 Guess the Number Game!");
    println!("I'm thinking of a number between 1 and 20.");

    // Generate random number between 1 and 20
    let secret_number = rand::thread_rng().gen_range(1..=20);

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number!");
                continue;
            }
        };

        if guess < secret_number {
            println!("📉 Too small!");
        } else if guess > secret_number {
            println!("📈 Too big!");
        } else {
            println!("🎉 Correct! You guessed the number.");
            break;
        }
    }
}
