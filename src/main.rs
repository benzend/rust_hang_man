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

        if word.contains(guessed_letter.trim()) {
            println!("nice!");
        } else {
            println!("oops!");
        }

        
        guessed_letters.push_str(&guessed_letter.trim());

        for cw in word.chars() {
           if found_letter_in(&guessed_letters, cw) {
               print!("{} ", cw);
           } else {
               print!("_ ");
           }
        }
        println!("");

        println!("Bad letters: ");
        for cw in guessed_letters.chars() {
            if !found_letter_in(&String::from(word), cw) {
                print!("{} ", cw);
            }
        }
        println!("")


    }

    


}

fn good_bye() {
    println!("Good bye!");
    std::process::exit(0);
}

fn found_letter_in(string: &String, c: char) -> bool {
    for cc in string.chars() {
        if cc == c {
            return true;
        } 
    }

    false
}