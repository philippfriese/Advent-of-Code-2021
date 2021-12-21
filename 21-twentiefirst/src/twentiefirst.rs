use core::iter::Map;
use std::convert::TryInto;
use cached::proc_macro::cached;
use itertools::Itertools;

pub fn first(content: &str) -> u16 {
    // solved using Spreadsheets
    0
}



// Note: This solution is heavily inspired by /u/miran1
fn dice_occurrences()-> Vec<(i8,i128)> {
    let oc =  (1..=3).cartesian_product(1..=3).cartesian_product(1..=3).map(|x| x.0.0 + x.0.1 + x.1).collect::<Vec<i8>>();
    oc.iter().unique().map(|x| (*x,oc.iter().filter(|y| **y==*x).count() as i128))
        .collect()
}


#[cached]
fn run(position_a: i8, position_b: i8, score_a: i8, score_b: i8) -> (i128,i128){
    if score_a >= 21 { return (1,0); }
    if score_b >= 21 { return (0,1); }
    
    let mut score = (0,0);

    for (roll,frequency) in dice_occurrences() {
        let new_position_a = (position_a + roll - 1) %  10 + 1;
        let new_score_a = new_position_a + score_a;
        let (score_b_, score_a_) = run(position_b, new_position_a, score_b, new_score_a);
        score.0 += score_a_ * frequency;
        score.1 += score_b_ * frequency;
    }
    score 
}


pub fn second(content: &str) -> i128 {
    let positions = content.split("\n").map(|x| x.chars().nth(28).unwrap().to_digit(10).unwrap() as i8).collect::<Vec<i8>>();

    let (p1,p2) = run(positions[0], positions[1],0,0);
    std::cmp::max(p1,p2)
}
