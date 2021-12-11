use itertools::{Itertools, enumerate};


pub fn first(content: &str) -> u32 {
    let open_chars = ['(', '[', '{', '<'];
    let close_chars = [')', ']', '}', '>'];
    let syntax_errors: Vec<u32> = content
        .split("\n")
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            for c in line.chars() {
                if open_chars.contains(&c) {
                    stack.push(c);
                }
                else {
                    let item = stack.pop().unwrap();
                    let correct = match item {
                        '(' => c == ')',
                        '[' => c == ']',
                        '{' => c == '}',
                        '<' => c == '>',
                        _ => panic!("Oh no")
                    };
                    if !correct {
                       return match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0
                        };
                    }
                }
            }
            0
        }).collect();
    syntax_errors.iter().sum::<u32>()
}

pub fn second(content: &str) -> u64 {
    let open_chars = ['(', '[', '{', '<'];
    let close_chars = [')', ']', '}', '>'];
    let mut scores: Vec<u64> = content
        .split("\n")
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();

            for c in line.chars() {
                if open_chars.contains(&c) {
                    stack.push(c);
                }
                else {
                    let item = stack.pop().unwrap();
                    let correct = match item {
                        '(' => c == ')',
                        '[' => c == ']',
                        '{' => c == '}',
                        '<' => c == '>',
                        _ => panic!("Oh no")
                    };
                    if !correct {
                       return 0;
                    }
                }
            }

            if stack.len() != 0 {
                let mut score = 0;
                for i in 1..=stack.len() {
                    let c = stack[stack.len()-i];
                    let s = match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Oh no")
                    };
                    score *= 5;
                    score += s;
                }
                return score;
            }
            0
        }).filter(|x| *x != 0).collect();
    scores.sort_unstable();
    scores[(scores.len()/2) as usize]
}
