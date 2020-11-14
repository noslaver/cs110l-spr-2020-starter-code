// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:

    let mut word = vec!['-'; secret_word.len()];
    let mut guesses = String::with_capacity(secret_word.len());
    // Your code here! :)
    loop {
        println!("The word so far is {}", &word.iter().collect::<String>());
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let c = guess.chars().nth(0).unwrap();
        if word.contains(&c) {
            continue;
        }

        if !secret_word_chars.contains(&c) {
            guesses.push(c);
            let len = guesses.len() as u32;
            if len == NUM_INCORRECT_GUESSES {
                println!("Sorry, you ran out of guesses!");
                break;
            }
            println!("You have guessed the following letters: {}", &guesses[..]);
            println!("You have {} guesses left", NUM_INCORRECT_GUESSES - len);
        }

        for (i, ch) in secret_word_chars.iter().enumerate() {
            if *ch == c {
                word[i] = c;
            }
        }

        if word == secret_word_chars {
            println!(
                "Congratulations you guessed the secret word: {}",
                &secret_word
            );
            break;
        }
    }
}
