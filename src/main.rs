use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: wordcount <file>");
        return;
    }
    let contents = fs::read_to_string(&args[1]).expect("Could not read file");
    let mut counts = HashMap::new();
    for word in contents.split_whitespace() {
        let w = word.to_lowercase();
        *counts.entry(w).or_insert(0) += 1;
    }
    for (word, count) in counts {
        println!("{}: {}", word, count);
    }
}