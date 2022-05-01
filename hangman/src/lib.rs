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
    },
    fn good_bye() {
        println!("Good bye!");
        std::process::exit(0);
    },
    fn hello() {
        println!("Welcome to hangman!");

        println!("Would you like to play? [yes/no]");

        let mut lets_play = String::new();

        io::stdin()
            .read_line(&mut lets_play)
            .expect("Not a string");

        if lets_play.trim() == "no" {
            good_bye();
        } else if lets_play.trim() == "yes" {
            println!("Let's go!");
        } else {
            good_bye();
        }
    }
}