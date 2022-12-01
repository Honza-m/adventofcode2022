use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn task1() {
    let mut v: Vec<usize> = Vec::new();

    let f = File::open("./src/day1/input.txt").expect("Couldn't open file");
    let lines = BufReader::new(f).lines();

    let mut cal_sum: usize = 0;
    for line in lines {
        if let Ok(cal) = line {
            match cal.as_str() {
                "" => {
                    v.push(cal_sum);
                    cal_sum = 0;
                }
                _ => cal_sum += cal.parse::<usize>().expect("Not a number!"),
            }
        }
    }

    println!("Most snacks - {}", v.iter().max().expect("Empty vector!"));

    v.sort();
    v.reverse();

    let top_three = v.drain(0..3);

    println!("Top three - {}", top_three.sum::<usize>())
}
