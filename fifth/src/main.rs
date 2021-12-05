use std::str::FromStr;
use std::fs;
use std::cmp;
use std::fmt;

#[derive(Debug)]
struct Line {
    from: (i32, i32),
    to: (i32, i32),
}

impl FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Line,()> {
        let mut split = s.split_ascii_whitespace();

        let mut from = split.next().unwrap().split(",");
        let arrow = split.next().unwrap();
        let mut to = split.next().unwrap().split(",");

        assert!(arrow == "->");

        let from_x = from.next().unwrap().parse().unwrap();
        let from_y = from.next().unwrap().parse().unwrap();

        let to_x = to.next().unwrap().parse().unwrap();
        let to_y = to.next().unwrap().parse().unwrap();

        Ok(Line {from:(from_x,from_y), to:(to_x, to_y)})
    }
}

impl Line {
    fn walk_range(&self) -> (i32,i32) {
        let walk_x = self.to.0 - self.from.0;
        let walk_y = self.to.1 - self.from.1;
        (walk_x, walk_y)
    }
}

struct Space {
    space: Vec<u32>,
    width: usize,
    height: usize
}

impl fmt::Display for Space {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height { // for each row
            let string_row = &self.space[y*self.width..(y+1)*self.width] // get row
                .iter()
                .map(|x| {
                    match x {
                        0 => ".".to_string(),
                        _ => x.to_string()
                    }
                })
                .collect::<Vec<String>>();
            write!(f, "{}\n", string_row.join(" "))?; // write it space-separated
        }

        Ok(())
    }
}

impl Space {
    fn new(width: usize, height: usize) -> Self {
        Self {space:vec![0;width*height],
              width:width, height:height}
    }

    fn coord_at(&self, x:usize, y:usize) -> usize {
        // x: row, y: col
        assert!(x < self.width);
        assert!(y < self.height);
        y*self.width + x
    }
    
    fn at(&self, x: usize, y:usize) -> u32 {
        return self.space[self.coord_at(x,y)];
    }


    fn add(&mut self, x: i32, y:i32) {
        let coord = self.coord_at(x.try_into().unwrap(), 
                                  y.try_into().unwrap());
        self.space[coord] += 1;
    }

    fn add_line(&mut self, line: &Line) {
        let walk_range = line.walk_range();
        if walk_range.0.abs() == walk_range.1.abs() || walk_range.0 == 0 || walk_range.1 == 0 {
            for i in 0..=cmp::max(walk_range.0.abs(), walk_range.1.abs()) {
                self.add(line.from.0 + i * walk_range.0.signum(), 
                         line.from.1 + i * walk_range.1.signum());
            }
        }
    }

    fn get_danger_spots(&self) -> Vec<(usize,usize)> {
        let mut danger_spots = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.at(x,y) >= 2 {
                    danger_spots.push((x,y));
                }
            }
        }
        danger_spots
    }
}

fn first(filename: &str, size: usize) -> usize{
    let mut space = Space::new(size,size);

    let data = fs::read_to_string(filename).expect("Oops");
    data
        .split("\n")
        .for_each(|x| {
            let line: Line = x.parse().unwrap();
            if line.from.0 == line.to.0 || line.from.1 == line.to.1 { // only allow diagonal and vertical lines
                space.add_line(&line);
            }
        });

    let danger_spots = space.get_danger_spots();

    danger_spots.len()
}

fn second(filename: &str, size: usize) -> usize{
    let mut space = Space::new(size,size);

    let data = fs::read_to_string(filename).expect("Oops");
    data
        .split("\n")
        .for_each(|x| {
            let line: Line = x.parse().unwrap();
            let walk_range = line.walk_range();
            if line.from.0 == line.to.0 || line.from.1 == line.to.1 ||
                walk_range.0.abs() == walk_range.1.abs() // allow diagonal, vertical and 45°-shifted lines
            {
                space.add_line(&line);
            }
        });

    let danger_spots = space.get_danger_spots();

    danger_spots.len()
}

fn main() {
    println!("Hello, world!");
    let result_first = first("data", 1000);
    println!("Result first: {}", result_first);

    let result_second = second("data", 1000);
    println!("Result second: {}", result_second);
}
