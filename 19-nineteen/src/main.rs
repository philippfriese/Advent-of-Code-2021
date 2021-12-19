use nineteen::nineteen::first;
use nineteen::nineteen::second;

use std::fs;
fn main() {
    let data = fs::read_to_string("test").expect("Oops");
    let result_first = first(&data);
    println!("Result first: {}", result_first);

    let result_second = second(&data);
    println!("Result second: {}", result_second);
}
