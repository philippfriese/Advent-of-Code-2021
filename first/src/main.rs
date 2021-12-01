use std::fs;

fn first(lines: &Vec<u32>) {


    let mut above: u32 = 0;
    let mut below: u32 = 0;
    for it in 1..lines.len() {
        let lhs = lines[it-1];
        let rhs = lines[it];
        above += (rhs > lhs) as u32;
        below += (rhs < lhs) as u32;
    }
    println!("above: {} below: {}", above, below);
}

fn second(lines: &Vec<u32>) -> u32 {
    // 0 0 199  1      
    // 1 1 200  A 2    
    // 2 2 208  A B 3  
    // 3 0 210    B C 1
    // 4 1 200  2   C D
    // 5 2 207  E 3   D
    // 6 0 240  E F 1  
    // 7 1 269    F G 2
    // 8 2 260      G H
    // 9 0 263        H
    // let mut acc1: u32 = 0; // 0,1,2
    // let mut acc2: u32 = 0; // 1,2,3
    // let mut acc3: u32 = 0; // 2,3,4 

    // let mut acc1_prev: u32 = 0;
    // let mut acc2_prev: u32 = 0;
    // let mut acc3_prev: u32 = 0;

    // let mut above: u32 = 0;
    // for it in 0..lines.len() {
    //     let value = lines[it];

    //     let mod_res = it % 3;
    //     match mod_res {
    //         0 => { acc1_prev = acc1; acc1 = 0;},
    //         1 => { acc2_prev = acc2; acc2 = 0;},
    //         2 => { acc3_prev = acc3; acc3 = 0;},
    //         _ => panic!("MATH IS BROKEN!")
    //     }

    //     acc1 += value;
    //     acc2 += value;
    //     acc3 += value;

    //     if it > 2 {
    //         match mod_res {
    //             0 => { above += (acc2 > acc1_prev) as u32 },
    //             1 => { above += (acc3 > acc2_prev) as u32 },
    //             2 => { above += (acc1 > acc3_prev) as u32 },
    //             _ => panic!("MATH IS BROKEN!")
    //         }
    //     }
    // }
    // return above;


    // Courtesy to Leon
    let mut above = 0;
    for index in 0..lines.len() - 3 {
        let (a, ab_1, ab_2, b) = (lines[index], lines[index + 1], lines[index + 2], lines[index + 3]);
        let left = a + ab_1 + ab_2;
        let right = ab_1 + ab_2 + b;
        above += (left < right) as u32;
    }
    return above;
}

fn main() {
    let data = fs::read_to_string("data").expect("Oops");
    let lines = data
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    first(&lines);
    second(&lines);
}
