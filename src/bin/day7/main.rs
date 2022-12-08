use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

struct FileRec {
    size: usize,
    name: String,
}

struct Directory {
    name: String,
    size: Option<usize>,
    contents: HashMap<String, FileType>,
}

enum FileType {
    File(FileRec),
    Dir(Directory),
}

fn main() {
    let f = File::open("./src/bin/day7/input.txt").expect("Couldn't open file");
    let lines = BufReader::new(f).lines();

    // Set up file system and current path
    let mut path: Vec<String> = Vec::new();
    let mut fs: HashMap<String, FileType> = HashMap::new();
    fs.insert(
        "/".to_string(),
        FileType::Dir(Directory {
            name: "/".to_string(),
            size: None,
            contents: HashMap::new(),
        }),
    );

    for line in lines {
        let s = line.unwrap();

        // Parse cd
        if s.starts_with("$ cd") {
            let path_dir = &s[4..s.len()];
            if path_dir == ".." {
                path.pop();
            } else {
                path.push(path_dir.to_owned());
            }
        } else if s.starts_with("$ ls") {
            continue;
        } else {
            let f: FileType = parse_file_type(&s);
            insert_to_fs(&mut fs, &path, f);
        }
    }
}

fn parse_file_type(s: &str) -> FileType {
    let s: Vec<&str> = s.split_whitespace().collect();
    match s[0] {
        "dir" => FileType::Dir(Directory {
            name: s[1].to_string(),
            size: None,
            contents: HashMap::new(),
        }),
        _ => FileType::File(FileRec {
            name: s[1].to_string(),
            size: s[0].parse().expect("Size not a number!"),
        }),
    }
}

fn insert_to_fs(fs: &mut HashMap<String, FileType>, path: &Vec<String>, f: FileType) {
    let mut current_folder = fs;
    for each in path {
        let x = match current_folder.get_mut(each) {
            Some(FileType::Dir(d)) => d,
            _ => continue,
        };
        current_folder = &mut x.contents;
    }

    let name = match f {
        FileType::File(f) => f.name,
        FileType::Dir(d) => d.name,
    };
    current_folder.insert(name, f);
}
