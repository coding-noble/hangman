#![deny(clippy::all)]

// import all dependencies
mod hangman;
mod words;

use hangman::Hangman;
use std::io::{self, Write};
use words::Words;

fn main() {
    let word_list: Vec<&str> = vec!["rust", "programming", "hangman"];
    let words: Words = Words::new(word_list);
    let secret_word: String = words.get_random_word();

    let mut hangman: Hangman = Hangman::new(&secret_word, 10);

    println!("Welcome to Hangman!");

    loop {
        println!("Word: {}", hangman.display_word());
        println!("Attempts left: {}", hangman.get_attempts_left());

        print!("Enter a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_char = guess.trim().chars().next().expect("Invalid input");
        if !hangman.make_guess(guess_char) {
            println!("You already guessed that letter!");
        }

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
