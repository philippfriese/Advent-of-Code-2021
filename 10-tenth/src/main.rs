use tenth::tenth::first;
use tenth::tenth::second;
use std::fs;
fn main() {
    let data = fs::read_to_string("data").expect("Oops");
    // let result_first = first(&data, 10);
    let result_first = first(&data);
    println!("Result first: {}", result_first);

    // let result_second = second(&data, 10, 5);
    let result_second = second(&data);
    println!("Result second: {}", result_second);
}
