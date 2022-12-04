use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let mut points: usize = 0;
    let f = File::open("./input.txt").expect("No file");
    let lines = BufReader::new(f).lines();
    for line in lines {
        if let Ok(s) = line {
            let v: Vec<&str> = s.split_whitespace().collect();
            let them: Move = match v[0] {
                "A" => Move::ROCK,
                "B" => Move::PAPER,
                "C" => Move::SCISSORS,
                _ => panic!("Unrecognised move"),
            };
            let result: Result = match v[1] {
                "X" => Result::LOOSE,
                "Y" => Result::DRAW,
                "Z" => Result::WIN,
                _ => panic!("Unrecognised result"),
            };

            points += (choose(&them, &result).points() + result.points()) as usize;
        }
    }

    println!("{}", points);
}

#[derive(Clone, Debug)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}
impl Move {
    fn points(&self) -> u8 {
        match self {
            Self::ROCK => 1,
            Self::PAPER => 2,
            Self::SCISSORS => 3,
        }
    }

    fn beats(&self) -> Move {
        match self {
            Self::ROCK => Self::SCISSORS,
            Self::PAPER => Self::ROCK,
            Self::SCISSORS => Self::PAPER,
        }
    }

    fn beaten_by(&self) -> Move {
        match self {
            Self::ROCK => Self::PAPER,
            Self::PAPER => Self::SCISSORS,
            Self::SCISSORS => Self::ROCK,
        }
    }
}

enum Result {
    WIN,
    DRAW,
    LOOSE,
}
impl Result {
    fn points(&self) -> u8 {
        match self {
            Self::WIN => 6,
            Self::DRAW => 3,
            Self::LOOSE => 0,
        }
    }
}

fn choose(them: &Move, result: &Result) -> Move {
    match result {
        Result::WIN => them.beaten_by(),
        Result::DRAW => them.clone(),
        Result::LOOSE => them.beats(),
    }
}
