use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Rock => write!(f, "Rock"),
            Move::Paper => write!(f, "Paper"),
            Move::Scissors => write!(f, "Scissors"),
        }
    }
}

impl Move {
    fn loses_to(&self) -> Move{
        match *self{
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock
        }
    }

    fn wins_against(&self) -> Move{
        match *self{
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Invalid move specified".to_string()),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("Invalid outcome specified".to_string()),
        }
    }
}

fn outcome_score(elf: Move, you: Move) -> u32 {
    if value_for(elf) % 3 == you as u32 {
        // win
        6
    } else if elf == you {
        // draw
        3
    } else {
        // loss
        0
    }
}

fn get_needed_move(elf:Move, outcome:Outcome) -> Move {
    match outcome{
        Outcome::Draw=> elf.clone(),
        Outcome::Lose => elf.wins_against(),
        Outcome::Win => elf.loses_to(),
    }
}

fn value_for(m: Move) -> u32 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

pub fn part1(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|line| {
            //for each line in the file
            let moves: Vec<Move> = line
                //split the line into elf and your moves and then map them to the enum
                .split(" ")
                .map(|s| Move::from_str(s).unwrap())
                .collect();
            // score from the outcome + score for move
            return outcome_score(moves[0], moves[1]) + value_for(moves[1]);
        })
        .sum();
    return result;
}

pub fn part2(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|line| {
            //for each line in the file
            let moves:Vec<&str> = line
                //split the line into elf and your moves and then map them to the enum
                .split(" ")
                .collect();

            let elf_move = Move::from_str(moves[0]).unwrap();
            let outcome = Outcome::from_str(moves[1]).unwrap();

            let your_move = get_needed_move(elf_move, outcome);

            // score from the outcome + score for move
            return outcome_score(elf_move, your_move) + value_for(your_move);
        })
        .sum();
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST_INPUT), 15)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST_INPUT), 12)
    }
}
