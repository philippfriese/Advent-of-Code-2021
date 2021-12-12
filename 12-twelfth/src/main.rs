use twelfth::twelfth::first;
use twelfth::twelfth::second;
use std::fs;
fn main() {
    let data = fs::read_to_string("data").expect("Oops");
    // let result_first = first(&data);
    // println!("Result first: {}", result_first);

    let result_second = second(&data);
    println!("Result second: {}", result_second);
}
