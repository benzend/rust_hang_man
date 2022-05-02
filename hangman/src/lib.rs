pub struct GameInstance {
    word: String,
    bad_attempts: u8,
    guessed_letters: String,
}

impl GameInstance {
    fn generate_new_word(&mut self) {
        self.word = String::from("Gutenberg");
    }
}

impl GameInstance {
    fn show_game_details(&self) {
        println!("Your word is {} letters long!", &self.word.len());
    }
}

impl GameInstance {
    fn show_filled_in_word(&self) {
        for c in self.word.to_lowercase().chars() {
            if helpers::found_letter_in(&self.guessed_letters, c) {
                print!("{} ", c);
            } else {
                print!("_ ");
            }
        }
    }
}

impl GameInstance {
    fn has_won(&self) -> bool {
        let mut empty_spaces: u8 = 0;

        for c in self.word.to_lowercase().chars() {
            if helpers::found_letter_in(&self.guessed_letters, c) {
            } else {
                empty_spaces += 1;
            }
        }

        empty_spaces == 0
    }
}

impl GameInstance {
    fn letter_matches(&self, letter: &String) -> bool {
        if self.guessed_letters.contains(letter.to_lowercase().trim()) {
            true
        } else {
            false
        }
    }
}

mod helpers {
    pub fn found_letter_in(string: &String, c: char) -> bool {
        for cc in string.chars() {
            if cc == c {
                return true;
            } 
        }
    
        false
    }
    fn good_bye() {
        println!("Good bye!");
        std::process::exit(0);
    }

    pub fn get_letter() -> String {
        println!("Input your number here");

        let mut guessed_letter = String::new();

        use std::io;

        io::stdin()
            .read_line(&mut guessed_letter)
            .expect("to be a string");
        let guessed_letter = guessed_letter.trim();

        if guessed_letter.len() > 1 
        || guessed_letter.len() == 0
        || guessed_letter.parse::<f64>().is_ok() {
            println!("You need to input a valid letter");
            return String::new();
        }

        String::from(guessed_letter)
    }
}

pub fn run_game(word: String) -> bool {
    let play = lets_play();

    if play {
        println!("Awesome, let's play!");
    } else {
        return true;
    }

    let mut game = GameInstance{word, bad_attempts: 0, guessed_letters: String::new()};

    game.show_game_details();

    loop {
        let guessed_letter = helpers::get_letter();

        let matches = game.letter_matches(&guessed_letter);

        if matches {
            println!("nice");
        } else {
            println!("bad");
        }
    }
}

fn lets_play() -> bool {
    println!("Welcome to hangman!");

    println!("Would you like to play? [yes/no]");

    let mut play = String::new();

    use std::io;

    io::stdin()
        .read_line(&mut play)
        .expect("Not a string");

    let play = play.trim();

    if play == "yes" {
        true
    } else {
        false
    }
}