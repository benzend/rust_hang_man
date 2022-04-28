use std::io;

fn main() {
    println!("Welcome to hangman!");

    println!("Would you like to play? [yes/no]");

    let mut lets_play = String::new();

    io::stdin()
        .read_line(&mut lets_play)
        .expect("Not a string");

    if lets_play == "no" {
        good_bye();
    } else if lets_play == "yes" {
        println!("Let's go!");
    }

    let word = "Wunderbar";

    let mut guessed_letters = String::new();

    println!("Your word is this {} letters long!", word.len());

    for i in 0..word.len() {
        if i == word.len() - 1 {
            println!("_");
        } else {
            print!("_ ");
        }
    }

    loop {
        let mut guessed_letter = String::new();

        println!("Try a letter");

        io::stdin()
            .read_line(&mut guessed_letter)
            .expect("to be a string");

        if letter_matches(&guessed_letter, &word).0 {
            println!("nice!");
        } else {
            println!("oops!");
        }

        guessed_letters.push_str(&guessed_letter.trim());
    }

    


}

fn good_bye() {
    println!("Good bye!");
    std::process::exit(0);
}

fn letter_matches(letter: &String, word: &str) -> (bool, String) {
    let bytes = word.as_bytes();

    for (_, &item) in bytes.iter().enumerate() {
        if item == letter.as_bytes()[0] {
            return (true, letter.to_string());
        }
    }

    (false, letter.to_string())
}