use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data/day_03.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut priority_sum = 0;

    for line in reader.lines() {
        let mut priority_count = [0u8; 52];

        let rucksack = line.expect("Could not parse line!");
        let size = rucksack.len();
        let (first, second) = rucksack.split_at(size / 2);

        for letter in first.chars() {
            let priority = match letter {
                'a'..='z' => letter as usize - 96,
                'A'..='Z' => letter as usize - 38,
                _ => panic!("Not an alphabetical character"),
            };
            priority_count[priority - 1] += 1;
        }

        for letter in second.chars() {
            let priority = match letter {
                'a'..='z' => letter as usize - 96,
                'A'..='Z' => letter as usize - 38,
                _ => panic!("Not an alphabetical character"),
            };
            if priority_count[priority - 1] >= 1 {
                priority_sum += priority;
                break;
            }
        }
    }

    println!("Sum of priority items is {}", priority_sum);

    // Just open the damn file again

    let file = File::open("data/day_03.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut badge_sum = 0;

    let mut elf_1 = [0u8; 52];
    let mut elf_2 = [0u8; 52];
    let mut elf_3 = [0u8; 52];

    for (linenum, line) in reader.lines().enumerate() {
        let rucksack = line.expect("Could not parse line!");

        for letter in rucksack.chars() {
            let priority = match letter {
                'a'..='z' => letter as usize - 96,
                'A'..='Z' => letter as usize - 38,
                _ => panic!("Not an alphabetical character"),
            };
            match linenum % 3 {
                0 => elf_1[priority - 1] += 1,
                1 => elf_2[priority - 1] += 1,
                2 => elf_3[priority - 1] += 1,
                _ => unreachable!("Modulo maths doesn't work like this?")
            }
        }

        // Check at the end of each group of three
        if (linenum % 3) == 2 {
            // Find letter that they have in common
            for i in 0..52 {
                if (elf_1[i] > 0) && (elf_2[i] > 0) && (elf_3[i] > 0) {
                    badge_sum += i + 1;
                    break
                }
            }

            // Reset elves for next group
            elf_1 = [0u8; 52];
            elf_2 = [0u8; 52];
            elf_3 = [0u8; 52];
        }
    }

    println!("The sum of badges is {}", badge_sum);
}
