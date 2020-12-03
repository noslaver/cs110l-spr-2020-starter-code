use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];

    let file = File::open(&filename1).expect("Unknown file");
    let mut reader = io::BufReader::new(file);

    let lines = reader.by_ref().lines().map(Result::unwrap);
    let lines_count = lines.count();

    println!("Lines: {}", lines_count);

    let file = File::open(&filename1).expect("Unknown file");
    let mut reader = io::BufReader::new(file);

    let lines = reader.by_ref().lines().map(Result::unwrap);
    let words_count: usize = lines.map(|l| l.split(' ').count()).sum();

    println!("Words: {}", words_count);

    let file = File::open(&filename1).expect("Unknown file");
    let mut reader = io::BufReader::new(file);

    let lines = reader.by_ref().lines().map(Result::unwrap);
    let chars_count: usize = lines
        .map(|l| l.split(' ').map(|w| w.chars()).flatten().count())
        .sum();

    println!("Chars: {}", chars_count);
}
