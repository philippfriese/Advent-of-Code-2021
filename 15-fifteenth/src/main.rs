use fifteenth::fifteenth::first;
use fifteenth::fifteenth::second;
use std::fs;
fn main() {
    let data = fs::read_to_string("data").expect("Oops");
    let result_first = first(&data, 100);
    println!("Result first: {}", result_first);

    let result_second = second(&data, 100);
    println!("Result second: {}", result_second);
}
