use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    println!("Advent of Code -- Day 01!");

    //Open the puzzle input
    let file = File::open("./input")?;
    //Create a buffered reader
    let reader = BufReader::new(file);

    // fixed sized array to count the top three elves calories
    let mut most_calories = [0, 0, 0];
    // accumulator
    let mut acc = 0;

    //Read each line
    for line in reader.lines() {
        let current_line = line?;
        // blank line means we're done counting the current elf's calories
        if current_line == "" {
            // does the current elf have more calories than the lowest of the top 3?
            if acc > most_calories[0] {
                // replace the lowest of the top 3 with the current elf
                most_calories[0] = acc;
                // resort the top 3 elves so lowest is in position 0
                most_calories.sort();
            }
            // reset the accumulator to zero for the next elf
            acc = 0;
        } else {
            // add the calories to the accumulator for the current elf
            acc = acc + current_line.parse::<i32>().unwrap();
        }
    }

    println!("The top three most calories carried are {:?}", most_calories);
    println!("Sum of top three: {}", most_calories.iter().sum::<i32>());

    Ok(())
}
