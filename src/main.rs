mod game_manager;

use std::io;
use std::fs;
use std::io::Write;
use std::process::exit;

use rand::seq::SliceRandom;

use crate::game_manager::*;

const WORDLIST: &'static str = "src/wordlist.txt";

fn main() {
    println!("Hangman v1.0 by sertdfyguhi.");

    // reads wordlist
    println!("Reading wordlist...");
    let contents = match fs::read_to_string(WORDLIST) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error when reading wordlist: {:?}", err);
            exit(1)
        }
    };
    let wordlist: Vec<&str> = contents.split('\n').collect();

    if wordlist.len() == 0 {
        println!("Empty wordlist found.");
        exit(0)
    }

    // get random word
    println!("Starting game...");
    let word = match wordlist.choose(&mut rand::thread_rng()) {
        Some(word) => word,
        None => unreachable!()
    };

    let mut manager = GameManager::new(word);

    loop {
        println!("{}", manager.to_hangman_str());

        let mut guess = String::new();

        print!("Guess: ");
        io::stdout().flush().expect("Unable to flush stdout.");
        io::stdin().read_line(&mut guess).expect("Unable to read stdin.");

        match manager.guess(guess.trim()) {
            GameStatus::Won => {
                println!("You won in {} tries!", manager.guesses);
                break;
            },
            GameStatus::Lost => {
                println!("You lost! The word was {}.", manager.word);
                break;
            },
            _ => continue,
        }
    }
}