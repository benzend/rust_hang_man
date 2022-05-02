struct GameInstance {
    word: str,
    bad_attempts: u8,
}

impl GameInstance {
    fn generate_new_word(&self) {
        &self.word = "Gutenberg";
    }
}

impl GameInstance {
    fn show_game_details(&self) {
        println!("Your word is {} letters long!", &self.word.len());
    }
}

mod helpers {
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
   
}

pub fn run_game(word str) -> (ended_early: bool) {
    let play = lets_play();

    if play {
        println!("Awesome, let's play!");
    } else {
        return (ended_early: true);
    }

    let mut game = GameInstance{word, bad_attempts: 0};
}

fn lets_play() -> bool {
    println!("Welcome to hangman!");

    println!("Would you like to play? [yes/no]");

    let mut play = String::new();

    io::stdin()
        .read_line(&mut play)
        .expect("Not a string");

    let play = &play.trim();

    if play == "yes" {
        true
    } else {
        false
    }
}