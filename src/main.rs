#![deny(clippy::all)]

// import all dependencies
mod hangman;
mod words;

use hangman::Hangman;
use std::io::{self, Write};
use words::Words;

fn main() {
    // Set up the list of words and the Hangman object.
    let words: Words = Words::new(vec![
        "programming",
        "rust",
        "code",
        "compiler",
        "ownership",
        "lifetime",
        "borrowing",
        "mutability",
        "trait",
        "concurrency",
        "async",
        "iterator",
        "closure",
        "dependency",
        "cargo",
        "macros",
    ]);
    let secret_word: String = words.get_random_word();

    let mut hangman: Hangman = Hangman::new(&secret_word, 10);

    println!("Welcome to Hangman!");

    // Main game logic
    loop {
        // Show the blank word and number of attempts left.
        println!("Word: {}", hangman.display_word());
        println!("Attempts left: {}", hangman.get_attempts_left());

        // Ask the user for a letter
        print!("Enter a letter: ");
        io::stdout().flush().unwrap();

        // Process there guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_char = guess.trim().chars().next().expect("Invalid input");
        if !hangman.make_guess(guess_char) {
            println!("You already guessed that letter!");
        }

        // Check to see if the user has guessed the word or is out of attempts.
        if hangman.is_game_over() {
            println!("Game over!");
            match hangman.is_word_guessed() {
                true => print!(
                    "Congratulations! You guessed the word: {}",
                    hangman.get_secret_word()
                ),
                false => print!(
                    "Sorry, you ran out of attempts. The word was: {}",
                    hangman.get_secret_word()
                ),
            }
            break;
        }
    }
}
