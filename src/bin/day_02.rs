use std::fs::File;
use std::io::{BufReader, BufRead};

enum Move {
    Rock     = 1,
    Paper    = 2,
    Scissors = 3
}

impl Move {
    fn from_str(string: &str) -> Self {
        match string {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Couldn't read move!")
        }
    }

    fn with_outcome(self, outcome: &Outcome) -> Self {
        use crate::{Outcome::*, Move::*};

        match (self, outcome) {
            (Rock, Win) | (Paper, Draw) | (Scissors, Lose) => Paper,
            (Scissors, Win) | (Rock, Draw) | (Paper, Lose) => Rock,
            (Paper, Win) | (Scissors, Draw) | (Rock, Lose) => Scissors
        }
    }
}

enum Outcome {
    Win  = 6,
    Draw = 3,
    Lose = 0
}

impl Outcome {
    fn from_str(string: &str) -> Self {
        match string {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Couldn't read outcome!")
        }
    }

    fn from_moves(their_move: &Move, your_move: &Move) -> Self {
        use crate::Move::*;

        match (their_move, your_move) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Self::Win,
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Self::Lose,
            _ => Self::Draw,
        }
    }
}

fn main() {
    let file = File::open("data/day_02.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut naive_score = 0;
    let mut proper_score = 0;

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if let Some((their_char, your_char)) = line.split_once(' ') {
            // Naive scoring (Part One)
            let their_move = Move::from_str(their_char);
            let your_move  = Move::from_str(your_char);
            naive_score += Outcome::from_moves(&their_move, &your_move) as i32 + your_move as i32;

            // Proper scoring (Part two)
            let the_outcome = Outcome::from_str(your_char);
            let your_move  = their_move.with_outcome(&the_outcome);
            proper_score += the_outcome as i32 + your_move as i32;
        }
    }

    println!("Your final (naive) score is {}", naive_score);
    println!("Your final (proper) score is {}", proper_score);
}