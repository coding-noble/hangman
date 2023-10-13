// The Hangman struct holds a secret word, the letters the user has guessed so far, and the number of attempts left.
pub struct Hangman {
    secret_word: String,
    guessed_letters: Vec<char>,
    attempts_left: usize,
}

impl Hangman {
    // Hangman constructor
    pub fn new(secret_word: &str, max_attempts: usize) -> Self {
        Hangman {
            secret_word: secret_word.to_lowercase(),
            guessed_letters: Vec::new(),
            attempts_left: max_attempts,
        }
    }

    // uses a for loop to display characters based on which letters have been guessed.
    pub fn display_word(&self) -> String {
        let mut display = String::new();
        for c in self.secret_word.chars() {
            display.push(if self.guessed_letters.contains(&c) {
                c
            } else {
                '_'
            });
        }
        display
    }

    // Logic behind making a guess
    pub fn make_guess(&mut self, guess: char) -> bool {
        let lowercase_guess = guess.to_lowercase().next().unwrap();
        if !self.guessed_letters.contains(&lowercase_guess) {
            self.guessed_letters.push(lowercase_guess);
            if !self.secret_word.contains(lowercase_guess) {
                self.attempts_left -= 1;
            }
            true
        } else {
            false
        }
    }

    // checks to see if the game is over.
    pub fn is_game_over(&self) -> bool {
        self.is_word_guessed() || self.attempts_left == 0
    }

    // checks to see if all the letters have been guessed
    pub fn is_word_guessed(&self) -> bool {
        self.secret_word
            .chars()
            .all(|c: char| self.guessed_letters.contains(&c))
    }

    // Getters
    pub fn get_secret_word(&self) -> &str {
        &self.secret_word
    }

    pub fn get_attempts_left(&self) -> usize {
        self.attempts_left
    }
}
