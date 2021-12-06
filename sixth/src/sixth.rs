use std::fs;

#[derive(Debug,Copy,Clone)]
struct Fish {
	age: i32,
	birthday: i32
}

pub fn first(filename: &str, num_days: i32) -> usize {
    let data = fs::read_to_string(filename).expect("Oops");
    let mut fish = data
        .split(",")
        .map(|x| { x.parse().unwrap() })
        .collect::<Vec<i32>>();

    for _day in 1..=num_days {
    	let mut new = Vec::<i32>::new();
    	fish = fish.iter().map(|age| {
			match *age-1 {
				-1 => {new.push(8); return 6}
				_ => {return *age-1}
			}
    	}).collect();
    	fish = [fish,new].concat();
    	//println!("Day {}: {:?}", day, fish);
    }

    fish.len()
}

// stolen from JaWeilBaum after depressing frustration
pub fn second(filename: &str, num_days: i32) -> usize {
    let data = fs::read_to_string(filename).expect("Oops");

    let mut stages = [0 as u64; 9];

    data
        .split(",")
        .for_each(|x| stages[x.parse::<usize>().unwrap()] += 1);

    for _ in 0..num_days {
    	let reproducing_fish = stages[0];
    	stages[0] = stages[1];
    	stages[1] = stages[2];
    	stages[2] = stages[3];
    	stages[3] = stages[4];
    	stages[4] = stages[5];
    	stages[5] = stages[6];
    	stages[6] = stages[7] + reproducing_fish;
    	stages[7] = stages[8];
    	stages[8] = reproducing_fish;
    }
    stages.iter().sum::<u64>() as usize
}