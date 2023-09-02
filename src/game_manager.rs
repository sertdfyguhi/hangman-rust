const HANGMAN1: &'static str = "⍥";
const HANGMAN2: &'static str = "╱│╲";
const HANGMAN3: &'static str = "╱╲";

// slices string with something something clamping idk its 1am
fn slice(string: &str, n: u8, min: u8) -> String {
    let str_len = string.chars().count();

    if n <= min {
        " ".repeat(str_len)
    } else {
        let end = (n.clamp(min, min + str_len as u8) - min) as usize;
        // using .chars since string contains UTF-16 characters
        // which in a string slice is not accounted for (slices at byte)
        string.chars().take(end).collect::<String>() + &" ".repeat(str_len - end)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Active,
    Striked,
    Won,
    Lost,
}

pub struct GameManager<'a> {
    pub word: &'a str,
    pub strikes: u8,
    pub guesses: u16,
    pub status: GameStatus,
    pos_bitmap: u16,
}

impl<'a> GameManager<'a> {
    pub fn new(word: &'a str) -> Self {
        GameManager {
            word,
            strikes: 0,
            guesses: 0,
            status: GameStatus::Active,
            pos_bitmap: 0 // 0b00000000,
        }
    }

    pub fn guess(&mut self, guess: &str) -> GameStatus {
        if guess.len() == 0 {
            return self.status
        }

        self.guesses += 1;

        // character guess
        if guess.len() != self.word.len() {
            let guess_c = guess.chars().next().unwrap();
            let matches: Vec<(usize, &str)> = self.word.match_indices(guess_c).collect();

            // no matches found means a strike
            if matches.len() == 0 {
                self.strikes += 1;
                self.status = if self.strikes > 5 {
                    GameStatus::Lost
                } else {
                    GameStatus::Striked
                };

                return self.status
            }

            for (i, _) in matches {
                // switch i'th bit in bitmap to 1
                self.pos_bitmap |= 1 << 15 - i;
            }

            self.status = if self.pos_bitmap == u16::MAX {
                GameStatus::Won
            } else {
                GameStatus::Active
            };
        } else { // word guess
            if self.word == guess {
                self.status = GameStatus::Won;
            } else {
                self.strikes += 1;
                self.status = if self.strikes > 5 {
                    GameStatus::Lost
                } else {
                    GameStatus::Striked
                };
            }
        }

        self.status
    }

    pub fn to_guessed_str(&self) -> String {
        self.word.chars().enumerate().map(|(i, c)| {
            // check if i'th bit in bitmap is 1
            if self.pos_bitmap & (1 << 15 - i) != 0 {
                c.to_string()
            } else {
                String::from("_")
            }
        }).collect::<Vec<String>>().join(" ")
    }

    pub fn to_hangman_str(&self) -> String {
        format!(
" ┌──────┐
 │      {}
 │     {}
 │      {}      {}
─┴─",
            slice(HANGMAN1, self.strikes, 0),
            slice(HANGMAN2, self.strikes, 1),
            slice(HANGMAN3, self.strikes, 4),
            self.to_guessed_str(),
        )
    }
}