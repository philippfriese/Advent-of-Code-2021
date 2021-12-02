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
        let split = s.split(" ").collect::<Vec<&str>>();
        let direction = split[0].to_ascii_lowercase();
        let amnt = split[1].parse::<u32>().unwrap();
        match direction.as_str() {
            "forward" => Ok(Direction::Forward(amnt)),
            "up" => Ok(Direction::Up(amnt)),
            "down" => Ok(Direction::Down(amnt)),
            _ => Err(()),
        }
    }
}

fn get_data(filename: &str) ->  Vec<Direction>{
    let data = fs::read_to_string(filename).expect("Oops");
    let lines = data
        .split("\n")
        .map(|x| x.parse::<Direction>().unwrap())
        .collect::<Vec<Direction>>();
    return lines
}

fn first(lines: &Vec<Direction>) {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    lines.iter().for_each(|x| {
        match x {
            Direction::Forward(amnt) => {horizontal += amnt;},
            Direction::Up(amnt) => {depth -= amnt;},
            Direction::Down(amnt) => {depth += amnt;}
        }
    });

    println!("Horizontal Position: {}, Depth: {}, Final: {}",
        horizontal, depth, horizontal*depth);
}

fn second(lines: &Vec<Direction>) {
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

    println!("Horizontal Position: {}, Depth: {}, Final: {}",
        horizontal, depth, horizontal*depth);
}

fn main() {
    let data = get_data("test");
    first(&data);
    second(&data);
}
