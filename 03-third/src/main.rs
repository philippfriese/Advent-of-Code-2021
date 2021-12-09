use std::fs;

fn first<const NUM_ACCS: usize>(filename: &str) ->  u32{
    let mut accumulators = [0 as i32; NUM_ACCS];
    let data = fs::read_to_string(filename).expect("Oops");
    data
        .split("\n")
        .for_each(|x| {
            for (i, c) in x.chars().enumerate() {
                accumulators[i] += 1- (2*(c == '0') as i32);
            }
        });

    let mut gamma: u32 = 0;

    for (idx, bit) in accumulators.iter().enumerate() {
        let mask = ((*bit > 0 ) as u32) << (NUM_ACCS - idx-1) as u32;
        gamma |= mask;
    }

    let mask = !(!0 << NUM_ACCS);

    let epsilon = (!gamma) & mask;
    return gamma * epsilon;
}

fn second<const NUM_ACCS: usize>(filename: &str) ->  u32{
    
    let data = fs::read_to_string(filename).expect("Oops");
    let mut lines_oxygen: Vec<Vec<bool>> = data
        .split("\n")
        .map(|x| {
            let mut acc = vec![false; NUM_ACCS];
            for (i, c) in x.chars().enumerate() {
                acc[i] = (c == '1') as bool;
            }
            acc
        }).collect();

    let mut lines_co2 = lines_oxygen.to_vec();
    for idx in 0..NUM_ACCS {
        let mut acc = 0;
        if lines_oxygen.len() > 1 {
            lines_oxygen.iter().for_each(|x| { acc += -1 + (2*x[idx] as i32); });
            lines_oxygen = lines_oxygen.into_iter().filter(|x| x[idx] == (acc >= 0)).collect::<Vec<Vec<bool>>>();
        }
        
        let mut acc = 0;
        if lines_co2.len() > 1 {
            lines_co2.iter().for_each(|x| { acc += -1 + (2*x[idx] as i32); });
            lines_co2 = lines_co2.into_iter().filter(|x| x[idx] == (acc < 0)).collect::<Vec<Vec<bool>>>();
        }   
    }


    
    let mut oxygen: u32 = 0;
    let mut co2: u32 = 0;

    for (idx, bit) in lines_oxygen[0].iter().enumerate() {
        let mask = (*bit as u32) << (NUM_ACCS - idx-1) as u32;
        oxygen |= mask;
    }
    for (idx, bit) in lines_co2[0].iter().enumerate() {
        let mask = (*bit as u32) << (NUM_ACCS - idx-1) as u32;
        co2 |= mask;
    }

    return oxygen * co2;
}

fn main() {
    let result_first = first::<12>("data");
    let result_second = second::<12>("data");
    println!("Result first: {}, second: {}", result_first, result_second);

}