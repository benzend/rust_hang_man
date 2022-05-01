struct GameInstance {
    word: str,
    bad_attempts: u8,
}

impl GameInstance {
    fn generate_new_word(&self) {
        &self.word = "Gutenberg";
    }
}

mod hangman {
    fn start_game(word str) -> GameInstance {
        GameInstance{word, bad_attempts: 0}
    },
    fn found_letter_in(string: &String, c: char) -> bool {
        for cc in string.chars() {
            if cc == c {
                return true;
            } 
        }
    
        false
    }
}