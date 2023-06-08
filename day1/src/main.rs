use std::{fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let mut lines = read_lines("./src/input.txt");
    lines.push("".to_string());

    let mut elves: Vec<i32> = Vec::new();
    let mut acc: i32 = 0;
    for line in lines.iter() {
        if line.is_empty() {
            elves.push(acc);
            acc = 0;
        } else {
            let numerical: i32 = line.parse().unwrap();
            acc += numerical;
        }
    }
    
    elves.sort();
    let sum: i32 = elves.iter().rev().take(3).sum();
    println!("{}", sum);
}
