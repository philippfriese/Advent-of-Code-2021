use fifth::fifth::first;
use fifth::fifth::second;

fn main() {
    const N: u32 = 1000;
    
    let start_1 = std::time::Instant::now();
    let mut result_1 = 0;
    let mut result_2 = 0;
    for _ in 0..N {
        result_1 = first("data");
    }

    let end_1 = std::time::Instant::now();


    let start_2 = std::time::Instant::now();

    for _ in 0..N {
        result_2 = second("data");
    }

    let end_2 = std::time::Instant::now();

    println!("Task 1: {:?}", end_1 - start_1);
    println!("Task 2: {:?}", end_2 - start_2);
    println!("Result 1: {}, Result 2: {}", result_1, result_2);
}

