use day_02::part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input").unwrap();
    println!("Answer is {}", part1(&file));
}