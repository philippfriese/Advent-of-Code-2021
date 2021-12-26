use core::ops::AddAssign;
use core::str::FromStr;
use std::ops::Add;
use std::fmt;

use regex::Regex;
use lazy_static::lazy_static;


#[derive(Debug,Clone)]
struct SnailNumber {
    s: String
}

impl fmt::Display for SnailNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.s)
    }
}

trait SnailArithmetic {
    fn explode(&mut self) -> bool;
    fn split(&mut self) -> bool;
    fn reduce(&mut self);

    fn magnitude(& self) -> i64;
}

impl SnailArithmetic for SnailNumber {
    fn explode(&mut self) -> bool{
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(\d+),(\d+)\]").unwrap();
        }

        let delta = self.s.chars()
            .scan((0,0), |state, x| { // for each char: keep track of open and closed brackets up to this char
                state.0 += (x=='[') as u32;
                state.1 += (x==']') as u32;
                Some(*state)
            })
            .map(|(o,c)| o-c)
            .collect::<Vec<_>>();

        let mut changed = false;
        let mut offset = 0;
        for m in RE.captures_iter(&self.s) {
            let main_match = m.get(0).unwrap();
            if delta[main_match.start()] > 4 {
                changed = true;
                let match_len = main_match.end() as i64 - main_match.start() as i64;
                offset -= match_len-1;

                let l_pos = main_match.start();
                let l_value = m.get(1).unwrap().as_str().parse::<i32>().unwrap();

                
                let r_pos = main_match.end();
                let r_value = m.get(2).unwrap().as_str().parse::<i32>().unwrap();

                self.s = format!("{}{}{}", self.s[0..l_pos].to_string(), 0, self.s[r_pos..].to_string());
                

                for c_i in (0..l_pos-1).rev() {
                    if self.s.chars().nth(c_i).unwrap().is_digit(10) {
                        let (l_idx, c) = match self.s.chars().nth(c_i-1).unwrap().is_digit(10) {
                            true =>  { (c_i - 1, self.s[c_i-1..=c_i].to_string()) }
                            false => { (c_i    , self.s[c_i  ..=c_i].to_string()) }
                        };

                        let mhs = format!("{}", c.parse::<i32>().unwrap() + l_value);
                        self.s = format!("{}{}{}",  self.s[0..l_idx].to_string(), mhs.to_string(), self.s[l_idx+c.len()..].to_string());
                        offset += mhs.len()  as i64;
                        break
                    }   
                }

                for c_i in 0..(self.s.len()-(r_pos-1)) {
                    let r_idx = (r_pos as i64 + c_i  as i64 + offset  as i64) as usize;
                    if self.s.chars().nth(r_idx).unwrap().is_digit(10) {
                        let c = match self.s.chars().nth(r_idx+1).unwrap().is_digit(10) {
                            true =>  { self.s[r_idx..=r_idx+1].to_string() }
                            false => { self.s[r_idx..=r_idx  ].to_string() }
                        };

                        let mhs = format!("{}", c.parse::<i32>().unwrap() + r_value);
                        self.s = format!("{}{}{}",  self.s[0..r_idx].to_string(), mhs.to_string(), self.s[r_idx+c.len()..].to_string());
                        break
                    }   
                }
                break;
            }
            
        }

        changed
    } 

    fn reduce(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            if self.explode() {
                changed = true;
                continue;
            }
            if self.split() {
                changed = true;
                continue;
            }
        }
    } 

    fn split(&mut self) -> bool{
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d{2}").unwrap();
        }

        let mut changed = false;

        for m in RE.find_iter(&self.s) {
            let d = (m.as_str()).parse::<f32>().unwrap();
            let inner = format!("[{},{}]", (d/2.).floor(), (d/2.).ceil());
            let lhs = self.s[0..m.start()].to_string();
            let rhs = self.s[m.end()..].to_string();
            self.s = format!("{}{}{}", lhs,inner,rhs);
            
            changed = true;
            break;
        }

        changed
    } 

    fn magnitude(& self) -> i64{
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(\d+),(\d+)\]").unwrap();
        }

        let mut found = true;

        let mut s = self.s.clone();
        while found {

            found = false;
            for m in RE.captures_iter(&s) {
                let main_match = m.get(0).unwrap();
                found = true;
                let left = m.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let right = m.get(2).unwrap().as_str().parse::<i64>().unwrap();

                s = format!("{}{}{}", 
                    s[..main_match.start()].to_string(), 
                    left*3+right*2,
                    s[main_match.end()..].to_string());
                break
            }
        } 
        s.parse().unwrap() 
    } 
}

impl FromStr for SnailNumber {
    type Err = ();
    fn from_str(s: &str) -> Result<SnailNumber,()> {
        Ok(SnailNumber{s: s.to_string()})
    }
} 

impl Add for SnailNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        SnailNumber {
            s: format!("[{},{}]", self.s, other.s)
        }
    }
}

impl AddAssign for SnailNumber {
    fn add_assign(&mut self, other: Self) {
        *self = SnailNumber {
            s: format!("[{},{}]", self.s, other.s)
        }
    }
}


pub fn first(content: &str) -> i64 {
    let numbers = content
        .split("\n")
        .map(|x| x.parse::<SnailNumber>().unwrap())
        .collect::<Vec<SnailNumber>>();


    let mut n = numbers[0].clone();
    for i in 1..numbers.len() {
        n += numbers[i].clone();
        n.reduce();
    }
    n.magnitude()
}


pub fn second(content: &str) -> i64 {
    let numbers = content
        .split("\n")
        .map(|x| x.parse::<SnailNumber>().unwrap())
        .collect::<Vec<SnailNumber>>();


    let mut max_mag = 0;
    
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {continue}
            let mut n = numbers[i].clone() + numbers[j].clone();
            n.reduce();
            max_mag = std::cmp::max(max_mag,  n.magnitude());
        }
    }

    max_mag
}