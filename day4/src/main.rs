use std::{fs::read_to_string};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn split_range(range: &str) -> Vec<i32> {
    return range.split('-').map(|s| s.parse::<i32>().unwrap()).collect();
}

fn main() {
    let lines = read_lines("./src/input.txt");
    
    let mut score = 0;
    for line in lines {
        let ranges: Vec<_> = line.split(',').collect();
        let range1: Vec<i32> = split_range(ranges[0]);
        let range2: Vec<i32> = split_range(ranges[1]);

        if !(range1[0] > range2[1] || range1[1] < range2[0]) {
            score += 1;
        }
    }
    println!("{}", score);
}
