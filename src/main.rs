mod game_manager;

use std::io::{self, Write};
use rand::seq::SliceRandom;
use crate::game_manager::*;

const WORDLIST_CONTENTS: &'static str = include_str!("wordlist.txt");

fn main() {
    println!("Hangman v1.0 by sertdfyguhi.");

    let wordlist: Vec<&str> = WORDLIST_CONTENTS.split('\n').collect();
    let word = wordlist.choose(&mut rand::thread_rng()).unwrap();

    let mut manager = GameManager::new(word);

    loop {
        println!("{}", manager.to_hangman_str());

        let mut guess = String::new();

        print!("Guess: ");
        io::stdout().flush().expect("Unable to flush stdout.");
        io::stdin().read_line(&mut guess).expect("Unable to read stdin.");

        match manager.guess(guess.trim()) {
            GameStatus::Won => {
                println!("You won in {} tries with {} strikes!", manager.guesses, manager.strikes);
                break;
            },
            GameStatus::Lost => {
                println!("You lost with {} tries! The word was {}.", manager.guesses, manager.word);
                break;
            },
            _ => continue,
        }
    }
}