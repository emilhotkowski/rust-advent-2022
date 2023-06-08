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
    let lines = read_lines("./src/input.txt");
    
    let mut score = 0;
    for line in lines {
        let length = line.len();
        let comp1 = &line[..(length/2)];
        let comp2 = &line[(length/2)..];
        
        let mut items1 = HashSet::new();
        items1.extend(comp1.chars());

        let mut items2 = HashSet::new();
        items2.extend(comp2.chars());

        let same_items = items1.intersection(&items2);

        for item in same_items {
            let ord = *item as i32;
            if ord <= ('Z' as i32) {
                score += ord - ('A' as i32) + 27;
            } else {
                score += ord - ('a' as i32) + 1;
            }
        }
    }
    println!("{}", score);
}
