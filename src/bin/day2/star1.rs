use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn star1() {
    let mut score: u32 = 0;

    let f = File::open("./src/bin/day2/input.txt").expect("No found file");
    let lines = BufReader::new(f).lines();
    for line in lines {
        if let Ok(s) = line {
            let v: Vec<&str> = s.split_whitespace().collect();
            let them: Move = match v[0] {
                "A" => Move::ROCK,
                "B" => Move::PAPER,
                "C" => Move::SCISSORS,
                &_ => panic!("Invalid request"),
            };
            let me: Move = match v[1] {
                "X" => Move::ROCK,
                "Y" => Move::PAPER,
                "Z" => Move::SCISSORS,
                &_ => panic!("Invalid request"),
            };
            let result = me.fight(&them);
            score += (result.points() + me.points()) as u32;
        }
    }

    println!("Final score - {}", score);
}

#[derive(Debug)]
enum GameResult {
    WIN,
    DRAW,
    LOSE,
}

impl GameResult {
    fn points(&self) -> u8 {
        match self {
            GameResult::WIN => 6,
            GameResult::DRAW => 3,
            GameResult::LOSE => 0,
        }
    }
}
#[derive(Debug, PartialEq)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Move {
    fn points(&self) -> u8 {
        match self {
            Move::ROCK => 1,
            Move::PAPER => 2,
            Move::SCISSORS => 3,
        }
    }

    fn nemesis(&self) -> Move {
        match self {
            Move::ROCK => Move::PAPER,
            Move::PAPER => Move::SCISSORS,
            Move::SCISSORS => Move::ROCK,
        }
    }

    fn fight(&self, them: &Move) -> GameResult {
        if self == them {
            GameResult::DRAW
        } else if them == &self.nemesis() {
            GameResult::LOSE
        } else {
            GameResult::WIN
        }
    }
}
