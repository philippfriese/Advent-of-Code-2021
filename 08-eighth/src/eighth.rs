use core::str::FromStr;
use std::collections::HashSet;

struct Measurement {
    observations: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Measurement{
    type Err = ();
    fn from_str(s:  &str) -> Result<Measurement, ()> {
        let mut split = s.split("|");
        let observations:Vec<String> = split.next().unwrap().split_ascii_whitespace().map(|x| x.to_string()).collect();
        let output:Vec<String> = split.next().unwrap().split_ascii_whitespace().map(|x| x.to_string()).collect();

        return Ok(Measurement {observations: observations, output: output});
    }

}

pub fn first(content: &str) -> i32 {
    let data: Vec<Measurement> = content
        .split("\n")
		.map(|x| {
            x.parse::<Measurement>().unwrap()
        }).collect();

    let mut counts = [0;4];
    for line in data {
        for observation in line.output {
            match observation.len() {
                2 => {counts[0]+=1},
                3 => {counts[2]+=1},
                4 => {counts[1]+=1},
                7 => {counts[3]+=1},
                _ => {}
            }
        }

    }

	counts.iter().sum()
}

pub fn second(content: &str) -> i64 {
    let data: Vec<Measurement> = content
        .split("\n")
        .map(|x| {
            x.parse::<Measurement>().unwrap()
        }).collect();

    let mut output_number: i64 = 0;

    for line in data {
        let mut chars_1 = HashSet::new();
        let mut chars_7 = HashSet::new();
        let mut chars_4 = HashSet::new();
        for observation in line.observations {
            match observation.len() {
                2 => { chars_1 = observation.chars().collect(); }, // 1
                3 => { chars_7 = observation.chars().collect(); }, // 7
                4 => { chars_4 = observation.chars().collect(); }, // 4
                7 => {},// chars_8 = observation.chars().collect(); }, // 8
                _ => {}
            }
        }

        /*
        Decode logic
        For digit D with 6 segments:
        if   D does not share one char with 1: 6
        elif D with char 4 lights up all segments: 0
        else: 9

        For digit D with 5 segments:
        if   D shares all chars with 7: 3
        elif D with char 4 lights up all segments: 2
        else: 5
        */
        let mut numbers = [0;4];
        for (i,output) in line.output.iter().enumerate() {
            match output.len() {
                2 => { numbers[i] = 1},
                3 => { numbers[i] = 7},
                4 => { numbers[i] = 4}, 
                7 => { numbers[i] = 8}, 
                5 => {
                    let chars: HashSet<char> = output.chars().collect();

                    if chars.intersection(&chars_7).count() == 3 { numbers[i] = 3; } 
                    else if chars.union(&chars_4).count() == 7 { numbers[i] = 2;}
                    else {numbers[i] = 5;}
                },
                6 => {
                    let chars: HashSet<char> = output.chars().collect();

                    if chars.intersection(&chars_1).count() == 1 { numbers[i] = 6; } 
                    else if chars.union(&chars_4).count() == 7 { numbers[i] = 0;}
                    else {numbers[i] = 9;}
                },
                _ => {}
            }
        }
        let number = 1000 * numbers[0] + 100 * numbers[1] + 10 * numbers[2] + numbers[3];
        output_number += number;
    }

	output_number
}