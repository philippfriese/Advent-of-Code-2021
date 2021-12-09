

pub fn first(content: &str) -> i32 {
    let positions: Vec<i32> = content
        .split(",")
		.map(|x| {
            x.parse::<i32>().unwrap()
        }).collect();

    let max_pos = positions.iter().max().unwrap();	
    let mut min_fuel = i32::MAX;

    for i in 0..*max_pos {
    	let required_fuel: i32 = positions.iter().map(|p| (p-i).abs() ).sum();
    	if required_fuel > min_fuel { break; } 
    	min_fuel = required_fuel;
    }

	min_fuel 
}

pub fn second(content: &str) -> i32 {
    let positions: Vec<i32> = content
        .split(",")
		.map(|x| {
            x.parse::<i32>().unwrap()
        }).collect();

    let max_pos = positions.iter().max().unwrap();	
    let mut min_fuel = i32::MAX;

    for i in 0..*max_pos {
    	let required_fuel = positions.iter().map(|p| {let n = (p-i).abs(); return (n * (n+1))/2} ).sum();
    	if required_fuel > min_fuel { break; } 
    	min_fuel = required_fuel;
    }

	min_fuel 
}