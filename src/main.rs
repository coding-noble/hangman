#![deny(clippy::all)]

mod words;
use words::Words;

fn main() {
    let word_bank = Words::new(vec!["enter", "words", "here"]);

    if let Some(random_word) = word_bank.get_random_word() {
        println!("{}", random_word);
    } else {
        println!("No words available");
    }
}
