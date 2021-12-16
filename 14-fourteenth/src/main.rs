use fourteenth::fourteenth::first;
use fourteenth::fourteenth::second;
use fourteenth::fourteenth::run_01;
use fourteenth::fourteenth::run_02;

use std::fs;
fn main() {
    let data = fs::read_to_string("data").expect("Oops");
    let result_first = first(&data, 10);
    println!("Result first: {}", result_first);

    let result_second = second(&data, 40);
    println!("Result second: {}", result_second);

        let result_first = run_01(&data);
    println!("Result run_01: {}", result_first);

    let result_second = run_02(&data);
    println!("Result run_02: {}", result_second);
}
