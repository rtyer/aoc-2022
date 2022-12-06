use day_02::part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input").unwrap();
    println!("Answer is {}", part2(&file));
}
