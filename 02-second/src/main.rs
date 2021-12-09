

use std::str::FromStr;
use std::fs;


#[derive(Debug)]
enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32)
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Direction, ()> {
        let mut split = s.split_ascii_whitespace();
        let direction = split.next().unwrap();
        let amnt = split.next().unwrap().parse::<u32>().unwrap();

        match direction {
            "forward" => Ok(Direction::Forward(amnt)),
            "up" => Ok(Direction::Up(amnt)),
            "down" => Ok(Direction::Down(amnt)),
            _ => Err(()),
        }
    }
}

fn get_data(filename: &str) -> Vec<Direction> {
    let data = fs::read_to_string(filename).expect("Oops");
    let lines = data
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();
    return lines
}

fn first(lines: &Vec<Direction>) -> u32 {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    lines.iter().for_each(|x| {
        match x {
            Direction::Forward(amnt) => {horizontal += amnt;},
            Direction::Up(amnt)      => {depth -= amnt;},
            Direction::Down(amnt)    => {depth += amnt;}
        }
    });

    // println!("Horizontal Position: {}, Depth: {}, Final: {}",
    //     horizontal, depth, horizontal*depth);
    return horizontal*depth;
}

fn second(lines: &Vec<Direction>) -> u32 {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;
    lines.iter().for_each(|x| {
        match x {
            Direction::Forward(amnt) => {horizontal += amnt; depth += aim * amnt;},
            Direction::Up(amnt) => {aim -= amnt;},
            Direction::Down(amnt) => {aim += amnt;}
        }
    });

    // println!("Horizontal Position: {}, Depth: {}, Final: {}",
    //     horizontal, depth, horizontal*depth);
    return horizontal*depth;
}


fn main() {
    let data = get_data("data");
    
    let result_first = first(&data);
    let result_second = second(&data);

    println!("First result: {}, Second result: {}", result_first, result_second);
    // const N:i32 = 100000;
    // let start = std::time::Instant::now();
    // let mut c = 0;
    // for _ in 0..N { 
        
    //     c += first(&data); 
    // }
    // println!("Elapsed own: {:?} {}", start.elapsed(), c);
   

}
