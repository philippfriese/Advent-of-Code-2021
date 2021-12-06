
use sixth::sixth::second;
use sixth::sixth::second_faster;
use sixth::sixth::first;
fn main() {
    // let result_first = first("data", 80);
    // println!("Result first: {}", result_first);
    let mut result_second = 0;
    for _ in 0..100000 {
        result_second = second_faster("data", 256);
    }
    println!("Result second: {}", result_second);
}
