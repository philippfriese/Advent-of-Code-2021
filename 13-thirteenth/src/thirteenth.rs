use core::str::FromStr;
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32)
}

impl FromStr for Fold {
    type Err = ();
    fn from_str(s: &str) -> Result<Fold,()> {
        let mut split = s.split("=");
        let dir = split.next().unwrap();
        let amnt = split.next().unwrap().parse::<u32>().unwrap();
        match dir {
            "x" => Ok(Fold::X(amnt)),
            "y" => Ok(Fold::Y(amnt)),
            _ => Err(())
        }
    }
} 

fn print_field(data: &Vec<(u32, u32)>, width: usize, height: usize) {
    let mut field = vec![vec!['.';width];height];
    for point in data {
        field[point.1 as usize][point.0 as usize] = '#';
    }
    field.iter().for_each(|row| println!("{}", row.iter().collect::<String>()));
}

fn write_file(data: &Vec<(u32, u32)>) -> std::io::Result<()> {
    let mut width = 0;
    let mut height = 0;
    for point in data {
        width = std::cmp::max(point.0, width);
        height = std::cmp::max(point.1, height);
    }
        
    let mut field = vec![vec!['.';(width+1) as usize];(height+1) as usize];
    for point in data {
        field[point.1 as usize][point.0 as usize] = '#';
    }
    
    let temp_directory = env::current_dir().unwrap();
    let temp_file = temp_directory.join("file");

    let mut file = File::create(temp_file).unwrap();

    for row in field {
        file.write(format!("{}\n", row.iter().collect::<String>()).as_bytes())?;
    }
    Ok(())
}

fn count_dots(data: &Vec<(u32, u32)>, width: usize, height: usize) -> usize {
    let mut field = vec![vec!['.';width];height];
    for point in data {
        field[point.1 as usize][point.0 as usize] = '#';
    }
    field.iter().map(|x|x.iter().filter(|y|**y=='#').count()).sum()
}

pub fn first(content: &str) -> usize {
    let mut data = Vec::new();
    let mut x_max = 0;
    let mut y_max = 0;

    let mut instructions = Vec::new();
    content
        .split("\n")
        .for_each(|x| {
            if x.len() != 0 {
                if x.contains("fold") {
                    let mut split = x.split(" ");
                    let fold: Fold = split.nth(2).unwrap().parse().unwrap();
                    instructions.push(fold);
                } else {
                    let mut split = x.split(",");
                    let x = split.next().unwrap().parse::<u32>().unwrap();
                    let y = split.next().unwrap().parse::<u32>().unwrap();
                    x_max = std::cmp::max(x,x_max);
                    y_max = std::cmp::max(y,y_max);
                    
                    data.push((x,y));  
                }
            }
        });

    for (i,instruction) in instructions.iter().enumerate() {
        println!("Instruction {}", i);
        for i in 0..data.len() {
            let point = data[i];
            match instruction {
                Fold::X(amnt) => {if point.0 > *amnt {data[i] = (point.0-2*(point.0-*amnt), point.1  ); x_max = std::cmp::max(data[i].0,x_max);}}
                Fold::Y(amnt) => {if point.1 > *amnt {data[i] = (point.0,   point.1-2*(point.1-*amnt)); y_max = std::cmp::max(data[i].1,y_max);}}
            }
        }
        break;
    }
    count_dots(&data, (x_max+1) as usize, (y_max+1) as usize)
}

pub fn second(content: &str) -> usize {
let mut data = Vec::new();
    let mut x_max = 0;
    let mut y_max = 0;

    let mut instructions = Vec::new();
    content
        .split("\n")
        .for_each(|x| {
            if x.len() != 0 {
                if x.contains("fold") {
                    let mut split = x.split(" ");
                    let fold: Fold = split.nth(2).unwrap().parse().unwrap();
                    instructions.push(fold);
                } else {
                    let mut split = x.split(",");
                    let x = split.next().unwrap().parse::<u32>().unwrap();
                    let y = split.next().unwrap().parse::<u32>().unwrap();
                    x_max = std::cmp::max(x,x_max);
                    y_max = std::cmp::max(y,y_max);
                    
                    data.push((x,y));  
                }
            }
        });

    for (i,instruction) in instructions.iter().enumerate() {
        println!("Instruction {}", i);
        for i in 0..data.len() {
            let point = data[i];
            match instruction {
                Fold::X(amnt) => {if point.0 > *amnt {data[i] = (point.0-2*(point.0-*amnt), point.1  )}}
                Fold::Y(amnt) => {if point.1 > *amnt {data[i] = (point.0,   point.1-2*(point.1-*amnt))}}
            }
        }
    }
    write_file(&data);
    0
}