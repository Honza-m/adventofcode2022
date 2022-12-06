use std::{collections::HashSet, fs::read_to_string};

fn main() {
    // assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
    // assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
    // assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
    // assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));

    let s = read_to_string("./src/bin/day6/input.txt").expect("Couldn't open file.");
    println!("Packet marker starts at {:?}", find_marker(&s, 4));
    println!("Message marker starts at {:?}", find_marker(&s, 14));
}

fn find_marker(s: &str, n: usize) -> Option<usize> {
    let s: Vec<char> = s.chars().collect();
    for i in n..s.len() {
        let marker = s[i - n..i].to_vec();
        let marker: HashSet<char> = HashSet::from_iter(marker);

        if marker.len() == n {
            return Some(i);
        }
    }

    return None;
}
