use twentieth::twentieth::both;

use std::fs;
fn main() {
    let data = fs::read_to_string("data").expect("Oops");
    let result_first = both(&data, 2);
    println!("Result first: {}", result_first);

    let result_second = both(&data, 50);
    println!("Result second: {}", result_second);
}
