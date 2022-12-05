use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elves: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./data/day_01.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut elf_resources = 0;
        for line in lines {
            match line {
                Err(_) => continue,
                Ok(text) if text == "" => {
                    elves.push(elf_resources);
                    elf_resources = 0
                },
                Ok(num) => { elf_resources += num.parse::<u32>().unwrap() }
            }
        }
    }

    elves.sort();
    elves.reverse();

    let max_resources = elves[0];
    let top_three: u32 = elves[0..3].iter().sum();

    println!("The maximum resources are: {max_resources}");
    println!("The top three resources are: {top_three}");
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}