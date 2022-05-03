use hangman;
use std::io;
fn main() {
    loop {
        hangman::run_game(String::from("Wunderbar"));

        let mut play_again = String::new();

        println!("Do you wish to play again? [yes/no]");
        io::stdin()
            .read_line(&mut play_again)
            .expect("This to be a string");
        
        if play_again.trim() == "yes" {
            continue;
        } else {
            break;
        }
    }
}