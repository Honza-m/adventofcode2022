use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}
impl Instruction {
    fn from_string(s: &String) -> Instruction {
        let chunks: Vec<&str> = s.split_whitespace().collect();
        let err_msg = "Expected a number!";
        let from: usize = chunks[3].parse().expect(err_msg);
        let to: usize = chunks[5].parse().expect(err_msg);
        Instruction {
            quantity: chunks[1].parse().expect(err_msg),
            from: from - 1,
            to: to - 1,
        }
    }
}

fn main() {
    let f = File::open("./src/bin/day5/input.txt").expect("Couldn't open file");
    let mut lines = BufReader::new(f).lines();

    // Get initial stacks
    let mut stacks: Vec<Vec<char>> = vec![
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    // Iterate through stack rows
    for _ in 0..8 {
        if let Ok(row) = lines.next().expect("Couldn't read line.") {
            // Iterate through stacks and construct
            let row_vec: Vec<char> = row.chars().collect();
            for i in 0..9 {
                let row_index = (i * 4) + 1;
                if row_vec[row_index] != ' ' {
                    stacks[i].push(row_vec[row_index]);
                }
            }
        }
    }

    // Reverse all vectors
    for i in 0..stacks.len() {
        stacks[i].reverse();
    }

    // Skip two lines
    lines.next();
    lines.next();

    for s in &stacks {
        println!("{:?}", s);
    }

    // Process instructions
    for line in lines {
        let line = line.unwrap();
        let i = Instruction::from_string(&line);

        // // STAR 1
        // for _ in 0..i.quantity {
        //     let temp = stacks[i.from].pop().expect("Stack empty!");
        //     stacks[i.to].push(temp);
        // }

        // STAR 2
        let l = stacks[i.from].len();
        let temp = stacks[i.from][l - i.quantity..l].to_vec();
        for _ in 0..i.quantity {
            stacks[i.from].pop();
        }
        for each in temp {
            stacks[i.to].push(each);
        }

        // for s in &stacks {
        //     println!("{:?}", s);
        // }
    }

    // Print results
    let mut res = String::new();
    for s in &mut stacks {
        res.push(s.pop().unwrap());
    }
    println!("{}", res);
}
