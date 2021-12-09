
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
    #[inline]
    fn walk_range(&self) -> (i32,i32) {
        (self.to.0 - self.from.0, 
         self.to.1 - self.from.1)
    }
}

struct Space<const WIDTH: usize, const HEIGHT: usize> {
    space: [[u32; WIDTH]; HEIGHT]
}

impl<const HEIGHT: usize, const WIDTH: usize> fmt::Display for Space<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..HEIGHT { // for each row
            let string_row = &self.space[y] // get row
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

impl<const HEIGHT: usize, const WIDTH: usize> Space<WIDTH, HEIGHT> {
    fn new() -> Self {
        Self {space: [[0; WIDTH]; HEIGHT]}
    }
    
    #[inline]
    fn at(&self, x: usize, y:usize) -> u32 {
        return self.space[x][y];
    }

    #[inline]
    fn add(&mut self, x: usize, y:usize) {
        self.space[x][y] += 1;
    }

    fn add_line(&mut self, line: &Line) {
        let walk_range = line.walk_range();
        if walk_range.0.abs() == walk_range.1.abs() || walk_range.0 == 0 || walk_range.1 == 0 {
            for i in 0..=cmp::max(walk_range.0.abs(), walk_range.1.abs()) {
                self.add((line.from.0 + i * walk_range.0.signum()).try_into().unwrap(), 
                         (line.from.1 + i * walk_range.1.signum()).try_into().unwrap());
            }
        }
    }

    fn get_danger_spots(&self) -> Vec<(usize,usize)> {
        let mut danger_spots = Vec::new();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.at(x,y) >= 2 {
                    danger_spots.push((x,y));
                }
            }
        }
        danger_spots
    }
}


pub fn first(filename: &str) -> usize{
    let mut space = Space::<1000,1000>::new();

    let data = fs::read_to_string(filename).expect("Oops");
    let lines = data
        .split("\n")
        .map(|x| { x.parse().unwrap() })
        .collect::<Vec<Line>>();

    lines
        .iter()
        .for_each(|line| {
            if line.from.0 == line.to.0 || line.from.1 == line.to.1 { // only allow diagonal and vertical lines
                space.add_line(&line);
            }});

    let danger_spots = space.get_danger_spots();

    danger_spots.len()
}

pub fn second(filename: &str) -> usize{
    let mut space = Space::<1000,1000>::new();

    let data = fs::read_to_string(filename).expect("Oops");
    let lines = data
        .split("\n")
        .map(|x| { x.parse().unwrap() })
        .collect::<Vec<Line>>();

    lines
        .iter()
        .for_each(|line| {
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
