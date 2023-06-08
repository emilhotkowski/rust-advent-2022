use std::{fs::read_to_string};
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let line = &read_lines("./src/input.txt")[0];
    
    let window_size = 14;
    for start in 0..(line.len()-window_size) {
        let mut letters: HashSet<char> = HashSet::new();
        for offset in 0..window_size {
            letters.insert(line.as_bytes()[start + offset] as char);
        }
        if letters.len() == window_size {
            println!("{}", start + window_size);
            break;
        }
    }
}
