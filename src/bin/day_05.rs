use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data/day_05.txt").expect("Could not open file!");
    let mut lines = BufReader::new(file).lines();

    let mut shipyard: Vec<VecDeque<char>> = Vec::new();

    let initial = lines
        .by_ref()
        .take_while(|line| !line.as_ref().unwrap().is_empty());

    for line in initial {
        let line = line.expect("Could not read line!");

        let blocks: Vec<String> = line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chars| chars.iter().collect::<String>())
            .collect();

        for (column, block) in blocks.iter().enumerate() {
            if column >= shipyard.len() {
                shipyard.push(VecDeque::new());
                println!("Shipyard now has {} lanes", shipyard.len());
            }

            let block = block.trim();
            if block.is_empty() || !block.starts_with('[') {
                continue
            }

            let value = block
                .strip_prefix("[").unwrap()
                .strip_suffix("]").unwrap()
                .chars().next().unwrap();

            println!("Block contained char {}", value);

            shipyard[column].push_back(value)
        }

    }

    for (lane, contents) in shipyard.iter().enumerate() {
        println!("Lane {}: {:?}", lane, contents);
    }

    for line in lines {
        let line = line.expect("Could not read line!");

        // Consume contents of line
        // Get number to move
        let line = line.strip_prefix("move ").expect("Line has no 'move' prefix!");
        let (num_to_move, line) = line.split_once(' ').unwrap();
        let line = line.strip_prefix("from ").expect("Line has no 'from' prefix!");
        let (from_lane, line) = line.split_once(' ').unwrap();
        let to_lane = line.strip_prefix("to ").expect("Line has no 'to' prefix");

        let num_to_move: usize = num_to_move.parse().unwrap();
        let from_lane: usize   = from_lane.parse().unwrap();
        let to_lane: usize     = to_lane.parse().unwrap();

        let mut crane_stack = VecDeque::new();

        for _ in 0..num_to_move {
            let container = shipyard[from_lane-1].pop_front().expect("No items to move from lane!");
            crane_stack.push_front(container);
        }

        for _ in 0..num_to_move {
            let container = crane_stack.pop_front().unwrap();
            shipyard[to_lane-1].push_front(container);
        }
    }

    for (lane, contents) in shipyard.iter().enumerate() {
        println!("Lane {}: {:?}", lane, contents);
    }

    let mut output = "".to_string();
    for lane in shipyard {
        output += &lane.front().unwrap().to_string();
    }

    println!("Answer: {}", output);
}
