use core::str::FromStr;
use std::fmt;

#[derive(Clone,Copy, PartialEq)]
enum Entry {
    EAST,
    SOUTH ,
    NONE
}

impl From<char> for Entry {
    fn from(c: char) -> Self {
        match c {
            '>' => Entry::EAST,
            'v' => Entry::SOUTH,
            '.' => Entry::NONE,
            _ => Entry::NONE
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entry::EAST => write!(formatter, ">"),
            Entry::SOUTH => write!(formatter, "v"),
            _ => write!(formatter, ".")
        }
    }
}

#[derive(Clone)]
struct Field {
    data: Vec<Vec<Entry>>
}

impl IntoIterator for Field {
    type Item = Vec<Entry>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Field {data: s
                .split("\n")
                .map(|line| line
                    .chars()
                    .map(|c| Entry::from(c))
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>()
        }
        )
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.iter().for_each(|row| {
            row.iter().for_each(|v| write!(f,"{}", v).unwrap());
            write!(f, "\n");
        });

        Ok(())
    }    
}

impl Field {
    fn step(&mut self) -> bool{
        let mut new = self.clone();
        let mut changed = false;

        self
            .data
            .iter()
            .enumerate()
            .for_each(|(x,row)|
                row
                    .iter()
                    .enumerate()
                    .for_each(|(y, v)| {
                        match v {
                            Entry::EAST => {
                                let pos = (y+1)%row.len();
                                if self.data[x][pos] == Entry::NONE {
                                    new.data[x][pos] = Entry::EAST;
                                    new.data[x][y] = Entry::NONE;
                                    changed = true;
                                }
                            }
                            _ => {}
                        }
                        
                    }));

        let mut new2 = new.clone();
        new
            .data
            .iter()
            .enumerate()
            .for_each(|(x,row)|
                row
                    .iter()
                    .enumerate()
                    .for_each(|(y, v)| {
                        match v {
                            Entry::SOUTH => {
                                let pos = (x+1)%new.data.len();
                                if new.data[pos][y] == Entry::NONE {
                                    new2.data[pos][y] = Entry::SOUTH;
                                    new2.data[x][y] = Entry::NONE;
                                    changed = true;
                                }
                            },
                            _ => {}
                        }
                    }));
        self.data = new2.data;
        changed
    }
}
pub fn first(content: &str) -> i64 {
    let mut field: Field = content.parse().unwrap();
    let mut c = 1;
    while field.step() { println!("{}", field); c+=1;}
    c
}
