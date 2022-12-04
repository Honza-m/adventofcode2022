use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LETTERS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn star1() {
    let mut priority: u32 = 0;
    // Load input
    let lines =
        BufReader::new(File::open("./src/day3/input.txt").expect("Couldn't open file")).lines();
    // Get line
    for ruc in lines {
        let ruc = ruc.expect("Couldn't read line");
        priority += process_rucsac(&ruc) as u32;
    }

    println!("Priority is {}", priority);
}

pub fn star2() {
    let mut priority: u32 = 0;
    // Load input
    let lines =
        BufReader::new(File::open("./src/day3/input.txt").expect("Couldn't open file")).lines();

    // Get groups of three
    let mut group: [HashSet<char>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];
    for (i, ruc) in lines.enumerate() {
        // Store three in a vector
        let x = ruc.expect("Couldn't read line");
        group[i % 3] = HashSet::from_iter(x.chars());

        // When i+1 % 3 == 0, process group and wipe
        if (i + 1) % 3 == 0 {
            priority += process_group(&group) as u32;
        }
    }

    println!("Priority - {}", priority);
}

fn get_priority(c: &char) -> usize {
    LETTERS
        .iter()
        .position(|x| x == c)
        .expect("Intersection not a letter")
        + 1
}

fn process_rucsac(ruc: &str) -> usize {
    // Split rucsac in two
    let comp_size = ruc.len() / 2;
    let (left, right) = ruc.split_at(comp_size);

    // Create two sets from two parts
    let left: HashSet<char> = HashSet::from_iter(left.chars());
    let right: HashSet<char> = HashSet::from_iter(right.chars());

    // Call intersection (guaranteed to only have one)
    let c = left.intersection(&right).next().expect("No intersection!");

    // Find value of intersection (position in LETTERS + 1)
    get_priority(c)
}

fn process_group(v: &[HashSet<char>; 3]) -> usize {
    // There is only one letter in common
    let c = v[0]
        .iter()
        .filter(|k| v[1].contains(k))
        .filter(|k| v[2].contains(k))
        .next()
        .expect("Wrong");

    // Find value of intersection
    get_priority(c)
}
