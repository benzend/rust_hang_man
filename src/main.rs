use hangman;
fn main() {
    hangman::run_game(String::from("Wunderbar"));
    // println!("Welcome to hangman!");

    // println!("Would you like to play? [yes/no]");

    // let mut lets_play = String::new();

    // io::stdin()
    //     .read_line(&mut lets_play)
    //     .expect("Not a string");

    // if lets_play.trim() == "no" {
    //     good_bye();
    // } else if lets_play.trim() == "yes" {
    //     println!("Let's go!");
    // } else {
    //     good_bye();
    // }

    // let word = "Wunderbar";
    // let word_lowercased = word.to_lowercase();

    // let mut bad_attempts = 0;
    // let mut guessed_letters = String::new();

    // println!("Your word is {} letters long!", word.len());
    // println!("");

    // for _ in 0..word.len() {
    //     print!("_ ");
    // }
    // println!("");
    // println!("");

    // loop {
    //     println!("Bad attempts: {}", bad_attempts);
    //     print!("Bad letters: ");
    //     for cw in guessed_letters.chars() {
    //         if !found_letter_in(&String::from(word).to_lowercase(), cw) {
    //             print!("{} ", cw);
    //         }
    //     }
    //     println!("");
    //     println!("");

    //     let mut guessed_letter = String::new();

    //     println!("Try a letter");
    //     println!("");

    //     io::stdin()
    //         .read_line(&mut guessed_letter)
    //         .expect("to be a string");


    //     if guessed_letter.trim().len() > 1 
    //     || guessed_letter.trim().len() == 0
    //     || guessed_letter.trim().parse::<f64>().is_ok() {
    //         println!("You need to input a valid letter");
    //         continue;
    //     }

        
    //     if guessed_letters.contains(guessed_letter.to_lowercase().trim()) {
    //         println!("You already tried that letter before");
    //         println!("");
    //         continue;
    //     }

    //     if word.to_lowercase().contains(guessed_letter.to_lowercase().trim()) {
    //         println!("nice!");
    //         println!("");
    //     } else {
    //         println!("oops!");
    //         println!("");
    //         bad_attempts += 1;

    //         if bad_attempts == 10 {
    //             println!("You lost!");
    //             good_bye();
    //         }
    //     }
        
    //     guessed_letters.push_str(guessed_letter.to_lowercase().trim());

    //     let mut empty_spaces: u8 = 0;

    //     for cw in word_lowercased.chars() {
    //         if found_letter_in(&guessed_letters, cw) {
    //             print!("{} ", cw);
    //         } else {
    //             print!("_ ");
    //             empty_spaces += 1;
    //         }
    //     }
    //     println!("");
    //     println!("");
        
    //     if empty_spaces == 0 {
    //         println!("You win!");
    //         good_bye();
    //     }


    // }

    


}