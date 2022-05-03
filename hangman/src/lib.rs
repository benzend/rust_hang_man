pub struct GameInstance {
    word: String,
    bad_attempts: u8,
    guessed_letters: String,
    max_attempts: u8
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
    fn guess_attempt(&mut self, guess: &String) -> bool {
        self.guessed_letters.push_str(&guess);
        if self.letter_matches(guess) {
            true 
        } else {
            self.bad_attempts += 1;
            false
        }
    }
}

impl GameInstance {
    fn letter_matches(&self, letter: &String) -> bool {
        if self.word.to_lowercase().contains(letter.to_lowercase().trim()) {
            true
        } else {
            false
        }
    }
}

pub struct ValidLetterRes {
    letter: String,
    ok: bool
}

mod helpers {
    use crate::ValidLetterRes;

    pub fn found_letter_in(string: &String, c: char) -> bool {
        for cc in string.chars() {
            if cc == c {
                return true;
            } 
        }
    
        false
    }

    pub fn get_letter() -> ValidLetterRes {
        println!("Input your number here");

        let mut guessed_letter = String::new();

        use std::io;

        io::stdin()
            .read_line(&mut guessed_letter)
            .expect("to be a string");

        let valid = is_valid_letter(&guessed_letter);

        ValidLetterRes{letter: String::from(&guessed_letter), ok: valid}
    }

    pub fn is_valid_letter(letter: &String) -> bool {
        println!("len {}, is_ok {}", letter.len(), letter.parse::<f64>().is_ok());
        let letter = letter.trim();

        if letter.len() > 1 
        || letter.len() == 0
        || letter.parse::<f64>().is_ok() {
            println!("You need to input a valid letter");
            return false;
        }

        true
    }
}

pub struct GameRes {
    pub ended_early: bool,
    pub won: bool
}

pub fn run_game(word: String) -> GameRes {
    let play = lets_play();

    if play {
        println!("Awesome, let's play!");
    } else {
        return GameRes{ ended_early: true, won: false };
    }

    let mut game = GameInstance{
        word, 
        bad_attempts: 0, 
        guessed_letters: String::new(),
        max_attempts: 10
    };

    game.show_game_details();

    let res: GameRes = loop {
        game.show_filled_in_word();

        let guessed_letter = helpers::get_letter();

        if !guessed_letter.ok {
            println!("Invalid letter, try again");
            continue;
        };

        let guessed_letter = String::from(&guessed_letter.letter);

        let correct = game.guess_attempt(&guessed_letter);

        if correct {
            println!("nice");
        } else {
            println!("bad");
        }

        println!("max {}, bad {}", game.max_attempts, game.bad_attempts);

        if game.bad_attempts == game.max_attempts {
            println!("You lost!");
            break GameRes{ ended_early: false, won: false };
        }

        if game.has_won() {
            println!("You won!");
            break GameRes{ ended_early: false, won: false };
        }
    };

    res
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