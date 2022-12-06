use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_unique_pattern(signal: &str, length: usize) -> Option<usize> {
    // With nightly, can use for window in signal.array_windows(length)
    for position in 0..(signal.len()-length) {
        let window = &signal[position..position+length];
        if HashSet::<char>::from_iter(window.chars()).len() == length {
            return Some(position + 14);
        }
    }
    None
}

fn main() {
    let file = File::open("./data/day_06.txt").expect("File not found!");
    let lines = BufReader::new(file).lines();

    // Only one line in this input...
    let signal = lines
        .into_iter()
        .next()
        .expect("No line found!")
        .expect("Could not parse line!");

    // Part 1
    match find_unique_pattern(&signal, 4) {
        Some(pos) => println!("Start-of-packet  found at {pos}"),
        None => println!("Could not find start-of-packet!")
    }

    // Part 2
    match find_unique_pattern(&signal, 14) {
        Some(pos) => println!("Start-of-message found at {pos}"),
        None => println!("Could not find start-of-message!")
    }
}
