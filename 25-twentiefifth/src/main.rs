use twentiefifth::twentiefifth::first;

use std::fs;
fn main() {
    let data = fs::read_to_string("data").expect("Oops");
    let result_first = first(&data);
    println!("Result first: {}", result_first);
}
