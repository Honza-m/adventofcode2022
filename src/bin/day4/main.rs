use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Range {
    min: u32,
    max: u32,
}
impl Range {
    fn from_string(s: &str) -> Self {
        let split_string: Vec<u32> = s
            .split("-")
            .map(|x| x.parse::<u32>().expect("Couldn't convert to number"))
            .collect();

        Self {
            min: split_string[0],
            max: split_string[1],
        }
    }

    fn fully_overlaps(&self, other: &Range) -> u32 {
        if (self.min <= other.min) && (self.max >= other.max) {
            1
        } else if (other.min <= self.min) && (other.max >= self.max) {
            1
        } else {
            0
        }
    }

    fn at_least_partially_overlaps(&self, other: &Range) -> u32 {
        if self.fully_overlaps(other) == 1 {
            1
        } else if self.min >= other.min && self.min <= other.max {
            1
        } else if self.max >= other.min && self.max <= other.max {
            1
        } else {
            0
        }
    }
}

fn main() {
    let mut fully_overlap: u32 = 0;
    let mut partially_overlap: u32 = 0;

    let f = File::open("./src/bin/day4/input.txt").expect("Couldn't open file.");
    let lines = BufReader::new(f).lines();
    for line in lines {
        let line = line.expect("Couldn't read line.");
        let ranges = line.split_once(",").expect("Invalide line format.");

        let first = Range::from_string(ranges.0);
        let second = Range::from_string(ranges.1);

        fully_overlap += first.fully_overlaps(&second);
        partially_overlap += first.at_least_partially_overlaps(&second);
    }

    println!("Fully - {}", fully_overlap);
    println!("At least partially - {}", partially_overlap);
}
