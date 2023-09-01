mod game_manager;

use std::io::{self, Write};
use std::process::exit;

use rand::seq::SliceRandom;

use crate::game_manager::*;

const WORDLIST_CONTENTS: &'static str = include_str!("wordlist.txt");

fn main() {
    println!("Hangman v1.0 by sertdfyguhi.");

    // reads wordlist
    println!("Loading wordlist...");
    let wordlist: Vec<&str> = WORDLIST_CONTENTS.split('\n').collect();

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