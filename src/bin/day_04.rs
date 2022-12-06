use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_range(range: &str) -> (isize, isize) {
    let (start_str, end_str) = range.split_once('-').unwrap();
    let start_num: isize = start_str.parse().unwrap();
    let end_num: isize = end_str.parse().unwrap();
    (start_num, end_num)
}

fn main() {
    let file = File::open("data/day_04.txt").expect("Could not read file!");
    let lines = BufReader::new(file).lines();

    let mut inlet_ranges = 0;
    let mut overlap_ranges = 0;

    for line in lines {
        let line = line.expect("Could not read line!");
        let (range_one, range_two) = line.split_once(',').unwrap();
        let (start_one, end_one) = parse_range(range_one);
        let (start_two, end_two) = parse_range(range_two);

        if (start_two >= start_one) && (end_two <= end_one)
            || (start_two <= start_one) && (end_two >= end_one)
        {
            inlet_ranges += 1;
        }

        if (start_two <= end_one) && (start_one <= end_two)
        {
            overlap_ranges += 1;
        }
    }

    println!("The number of completely inset ranges is {}", inlet_ranges);
    println!("The number of partially overlapped ranges is {}", overlap_ranges);
}
