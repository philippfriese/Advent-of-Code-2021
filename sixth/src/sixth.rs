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
pub fn second(data: &str, num_days: i32) -> usize {
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


pub fn second_faster(data: &str, num_days: usize) -> u64 {
    // this is actually a tiny bit faster than second
    // idea no. 1: second() essentially moves 9(+1) u64 values through arrays, 8 of which are unnecessary
    // only the datamove from `stages[6] = stages[7] + reproducing_fish` does "the work"
    // instead of moving around data needlessly, keep all 9 stages in the same position and instead determine
    //  which value-pair has to be added and moved for each iteration
    // however: doing this involves two mod-calls and leads to unpredictable data and instruction accesses
    //  so the raw performance will be _worse_ compared to second(). second() is straight-forward and all stages are probably cached anyway
    // idea no. 2: pre-calculate the movement pattern for one "cycle", i.e. for 9 value-moves and perform them in bulk.
    //  So determine the number of "cycles" and the remainder (num_days may not divide neatly). 
    //  Use the remainder to fall back to the old "determine the indices"-pattern.

    // For 80 and 256 iterations, this is barely faster (1.02x and 1.06x) than second(), for 2048 iterations it is about 1.5x faster
    // (we are already talking about ~2, ~2, and ~4us per function iteration for 80,256,2048 iterations, so optimisation is not that trivial)
    
    // note: I deliberately moved the file system access outside this function to avoid data-load jitter impacting the benchmark + caches
    
    let mut stages = [0 as u64; 9];
    
    data
        .split(",")
        .for_each(|x| stages[x.parse::<usize>().unwrap()] += 1);

    let (num_cycles, remaining) = (num_days / 9, num_days % 9);
    for _ in 0..num_cycles {
        stages[7] = stages[0] + stages[7];
        stages[8] = stages[1] + stages[8];
        stages[0] = stages[2] + stages[0];
        stages[1] = stages[3] + stages[1];
        stages[2] = stages[4] + stages[2];
        stages[3] = stages[5] + stages[3];
        stages[4] = stages[6] + stages[4];
        stages[5] = stages[7] + stages[5];
        stages[6] = stages[8] + stages[6];
    }
    for i in 0..remaining {
        let from = i % 9;
        let to = (from + 7) % 9;
        stages[to] = stages[from] + stages[to];
    }

    stages.iter().sum::<u64>()
}