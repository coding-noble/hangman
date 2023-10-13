use rand::prelude::SliceRandom;

// Words contains a vector to hold the strings to be picked from
pub struct Words {
    word_list: Vec<&'static str>,
}

// impl stands for Implement
impl Words {
    // new function acts as the constructor for the Words struct
    pub fn new(word_list: Vec<&'static str>) -> Self {
        Words { word_list }
    }

    // get_random_word uses the rand library to selects a random word from the vector in Words
    pub fn get_random_word(&self) -> String {
        let mut rng = rand::thread_rng();
        match self.word_list.choose(&mut rng) {
            Some(word) => word.to_string(),
            None => panic!("No words in the list!"),
        }
    }
}
