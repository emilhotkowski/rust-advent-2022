use std::{fs::read_to_string, collections::HashMap};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[derive(Debug)]
struct Result {
    opponent:  String,
    me:        String,
    score:     i32
}

fn main() {
    let lines = read_lines("./src/input.txt");
    let possible_results: Vec<Result> = vec![
        Result { opponent: "A".to_string(), me: "X".to_string(), score: 3},
        Result { opponent: "A".to_string(), me: "Y".to_string(), score: 6},
        Result { opponent: "A".to_string(), me: "Z".to_string(), score: 0},
        Result { opponent: "B".to_string(), me: "X".to_string(), score: 0},
        Result { opponent: "B".to_string(), me: "Y".to_string(), score: 3},
        Result { opponent: "B".to_string(), me: "Z".to_string(), score: 6},
        Result { opponent: "C".to_string(), me: "X".to_string(), score: 6},
        Result { opponent: "C".to_string(), me: "Y".to_string(), score: 0},
        Result { opponent: "C".to_string(), me: "Z".to_string(), score: 3},
    ];
    let const_points = HashMap::from([ 
        ("X", 1),
        ("Y", 2),
        ("Z", 3)
    ]);

    let mut score = 0;
        
    for line in lines.iter() {
        let choices: Vec<_> = line.split(' ').collect();
        let opponent = choices[0].to_string();
        let me = choices[1].to_string();
        
        score += const_points.get(me.as_str()).unwrap();
  
        let result = possible_results.iter()
            .find(|res| res.opponent == opponent && res.me == me)
            .unwrap();
        score += result.score;
        
    }
    println!("{}", score)
}
