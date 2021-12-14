use fourteenth::fourteenth::first;
use fourteenth::fourteenth::second;
use std::fs;
fn main() {
    let data = fs::read_to_string("data").expect("Oops");
    let result_first = first(&data, 10);
    println!("Result first: {}", result_first);

    let result_second = second(&data, 40);
    println!("Result second: {}", result_second);
}
