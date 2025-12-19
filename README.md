# Simple-program-in-Rust
Guess the Number – Rust CLI Game

Welcome to Guess the Number, a simple Rust command-line game where the player tries to guess a number between 1 and 20. This project is part of the Moringa AI Capstone Beginner Toolkit.

🚀 Project Overview

This project demonstrates:

How to set up a Rust development environment

How to create a minimal runnable CLI project

Use of the rand crate for random number generation

Handling user input and basic game logic in Rust

End Goal: Build a fun, interactive CLI game that beginners can run immediately.

🛠️ Prerequisites

Before you start, ensure you have:

Rust & Cargo installed: https://www.rust-lang.org/tools/install

Terminal / Command Prompt

VS Code or any code editor

Check installation:

rustc --version
cargo --version

⚙️ Setup Instructions
1. Create a New Project
cargo new guess_number
cd guess_number

2. Add the rand Crate

Edit Cargo.toml and add:

[dependencies]
rand = "0.8"


Or run:

cargo add rand

3. Update src/main.rs with Game Code
use std::io;
use rand::Rng;

fn main() {
    println!("🎯 Guess the Number Game!");
    println!("I'm thinking of a number between 1 and 20.");

    let secret_number = rand::thread_rng().gen_range(1..=20);

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

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

💻 Run the Game
cargo run


Example Output:

🎯 Guess the Number Game!
I'm thinking of a number between 1 and 20.
Please enter your guess:
10
📈 Too big!
Please enter your guess:
5
📉 Too small!
Please enter your guess:
7
🎉 Correct! You guessed the number.

🧠 Learning with GenAI

During development, AI prompts were used to:

Explain Rust basics to beginners

Scaffold project setup commands (cargo new, cargo run)

Generate random numbers using rand

Handle input validation with match

Reflection:
AI reduced trial-and-error and acted as a real-time tutor, helping me understand Rust concepts faster.

⚠️ Common Issues & Fixes
Issue	Solution
use of undeclared crate rand	Add rand = "0.8" to Cargo.toml or run cargo add rand
Program crashes on non-number input	Handled with match to validate input
Cargo command not found	Ensure Rust is installed and $HOME/.cargo/bin is in your PATH
📚 References

Rust Official Documentation

Rust Book – Beginner Friendly

Rand Crate Docs

Rust by Example

🎉 Contributing

Feel free to:

Improve the game (e.g., add scoring, limited attempts)

Add tests or new features

Share improvements via pull requests

🏷️ License

This project is MIT Licensed – feel free to use and adapt for educational purposes.
