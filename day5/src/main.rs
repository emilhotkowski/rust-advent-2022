use std::{fs::read_to_string};
use std::str::FromStr;
use std::collections::LinkedList;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[derive(Debug)]
struct Move {
    from:  i32,
    to:    i32,
    count: i32
}
#[derive(Debug)]
struct Input {
    stacks: Vec<LinkedList<char>>,
    moves:  Vec<Move>
}

fn parse_moves(lines: Vec<String>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        if line.starts_with("move") {
            let groups = move_regex.captures(&line).unwrap();
            let a_move = Move {
                from:   <i32 as FromStr>::from_str(groups.get(2).unwrap().as_str()).unwrap(),
                to:     <i32 as FromStr>::from_str(groups.get(3).unwrap().as_str()).unwrap(),
                count:  <i32 as FromStr>::from_str(groups.get(1).unwrap().as_str()).unwrap()
            };
            moves.push(a_move);
        }
    }
    return moves;
}

fn parse_stacks(lines: Vec<String>) -> Vec<LinkedList<char>> {
    let number_of_stacks = (lines[0].len() + 1) / 4;
    let line_of_stacks = lines.iter()
        .enumerate()
        .find(|(_, s)| s.is_empty())
        .unwrap().0 - 1;

    let mut stacks: Vec<LinkedList<char>> = Vec::new();
    for stack_idx in 0..number_of_stacks {
        let stack_idx_in_line = 1 + stack_idx * 4;
        let mut new_stack: LinkedList<char> = LinkedList::new();
        for row in (0..line_of_stacks).rev() {
            let ch = lines[row].as_bytes()[stack_idx_in_line] as char;
            if ch != ' ' {
                new_stack.push_back(ch);
            }
        }
        stacks.push(new_stack);
    }
    return stacks;
}

fn parse_input(lines: Vec<String>) -> Input {
    //Parse stacks
    let stacks = parse_stacks(lines.clone());
    //parse moves
    let moves = parse_moves(lines);

    Input {
        stacks,
        moves
    }
}

fn main() {
    let lines = read_lines("./src/input.txt");
    let input = parse_input(lines);

    let mut stacks = input.stacks;
    for a_move in input.moves {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..a_move.count {
            let element = stacks[(a_move.from - 1) as usize].pop_back().unwrap();
            temp.push(element);
        }
        for &element in temp.iter().rev() {
            stacks[(a_move.to - 1) as usize].push_back(element);
        }
    }
    for stack in stacks {
        print!("{}", stack.back().unwrap());
    }
    println!("");
}
