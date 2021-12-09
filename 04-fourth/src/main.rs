use std::str::FromStr;
use std::fs;


#[derive(Debug,Copy,Clone)]
struct Board {
    board: [[u32; 5]; 5],
    marked: [[bool;5];5],
    done: bool
}

impl FromStr for Board {
    type Err = ();
    fn from_str(s: &str) -> Result<Board, ()> {
        let mut board_data = [[0;5];5];
        let mut split = s.split_ascii_whitespace();
        for x in 0..5 {
            for y in 0..5 {
                let n = split.next().unwrap();
                let v = n.parse::<u32>().unwrap();
                board_data[x][y] = v;
            }
        }

        return Ok(Board {board: board_data, marked: [[false;5];5], done: false});
    }
}

impl Board {
    fn mark(&mut self, number: u32) {
        for x in 0..5 {
            for y in 0..5 {
                if self.board[x][y] == number {
                    self.marked[x][y] = true;
                }
            }
        }
    }

    fn check(&self) -> bool {
        let mut row_done = true;
        let mut col_done_vec = [true;5];

        for x in 0..5 {
            row_done = true;
            for y in 0..5 {
                let marked = self.marked[x][y];
                row_done = row_done && marked;
                col_done_vec[y] = col_done_vec[y] && marked;
            }

            if row_done { break }

        }
        row_done || col_done_vec.into_iter().reduce(|acc,it| acc||it).unwrap()
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for x in 0..5 {
            for y in 0..5 {
                if !self.marked[x][y] {
                    sum += self.board[x][y];
                }
            }
        }

        sum
    }
}

fn get_data(filename: &str) -> (Vec<u32>, Vec<Board>) {
    let data = fs::read_to_string(filename).expect("Oops");
    let board_blocks: Vec<String> = data
        .split("\n\n")
        .map(|x| x.replace("\n"," "))
        .collect();

    let boards = board_blocks[1..]  
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let numbers = board_blocks[0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    (numbers, boards)
}

fn first(numbers: &Vec<u32>, boards: &mut Vec<Board>) -> u32 {
    for n in numbers {
        for board in &mut *boards {
            board.mark(*n);
            let done = board.check();
            if done {
                let sum = board.unmarked_sum();
                return sum * n;
            }
        }
    }

    return 0;
}

fn second(numbers: &Vec<u32>, boards: &mut Vec<Board>) -> u32 {
    let mut last_sum = 0;
    for n in numbers {
        let mut c = 0;
        for mut board in &mut *boards {
            c+= 1;
            if !board.done {
                board.mark(*n);
                let check = board.check();
                if check {
                    board.done = true;
                    let sum = board.unmarked_sum();
                    last_sum = sum * n;
                }
            }
            
        }
    }

    return last_sum;
}
fn main() {
    let (numbers, boards) = get_data("data");
    let result_first = first(&numbers, &mut boards.clone());
    println!("{:?}", result_first);

    let result_second = second(&numbers, &mut boards.clone());
    println!("{:?}", result_second);
}