use rand::prelude::SliceRandom;
use rand::Rng;

pub struct Words {
    word_list: Vec<&'static str>,
}

impl Words {
    pub fn new(word_list: Vec<&'static str>) -> Self {
        Words { word_list }
    }

    // Static method to get a random word
    pub fn get_random_word(&self) -> Option<&'static str> {
        let mut rng = rand::thread_rng();
        self.word_list.choose(&mut rng).copied()
    }
}
