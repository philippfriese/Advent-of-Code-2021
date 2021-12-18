use core::str::FromStr;
use std::ops::Add;

#[derive(Debug,Clone)]
struct SnailNumber {
    value: Option<u32>,
    left: Option<Box<SnailNumber>>,
    right: Option<Box<SnailNumber>>
}

impl SnailNumber {
    fn print(&self) -> String {
        if !self.value.is_none() {
            return format!("{}", self.value.unwrap());
        }
        else { 
            return format!("[{},{}]", self.left.as_ref().unwrap().print(), self.right.as_ref().unwrap().print());
        }
        
    }


}

fn explode(number: SnailNumber, depth: u32, changed: &mut bool) -> SnailNumber {
    if !number.value.is_none() { return number; }

    if depth == 3 {
        *changed = true;
        if !number.right.as_ref().unwrap().value.is_none() {
            return SnailNumber {
                value: None,
                left: Some(Box::new(SnailNumber {value: Some(0),right:None, left:None})),
                right: Some(Box::new(SnailNumber { value: Some(number.left.unwrap().right.unwrap().value.unwrap() +  number.right.unwrap().value.unwrap()),
                                                   right:None, left:None})),
                
            };
        } else if !number.left.as_ref().unwrap().value.is_none() {
            return SnailNumber {
                value: None,
                left: Some(Box::new(SnailNumber { value: Some(number.right.unwrap().left.unwrap().value.unwrap() +  number.left.unwrap().value.unwrap()),
                                                  right:None, left:None})),
                right: Some(Box::new(SnailNumber {value: Some(0),right:None, left:None})),
                
            };
        }
        
    }
    else {
        return SnailNumber{
            value: number.value,
            left: Some(Box::new(explode(*number.left.unwrap(), depth+1, changed))),
            right: Some(Box::new(explode(*number.right.unwrap(), depth+1, changed)))
        };
    }

    return number; 
}


fn split(number: SnailNumber, changed: &mut bool) -> SnailNumber {
    if let Some(n) = number.value {
        if n > 9 {
            *changed = true;
           return SnailNumber {
                value: None,
                left:  Some(Box::new(SnailNumber {value: Some(((n as f32)/2.).floor() as u32), right:None,left:None})),
                right: Some(Box::new(SnailNumber {value: Some(((n as f32)/2.).ceil()  as u32), right:None,left:None})),
            } 
        } else { return number; }
    }
    return SnailNumber {
        value:number.value, 
        left: Some(Box::new(split(*number.left.unwrap(), changed))),
        right: Some(Box::new(split(*number.right.unwrap(), changed)))
    };
    
}

fn reduce(number: SnailNumber) -> SnailNumber {
    let mut changed = true;
    let mut n = number;
    while changed {
        changed = false;
        n = explode(n,0, &mut changed);
        n = split(n, &mut changed);
    }
    return n;
}

fn build(s: &str) -> SnailNumber {
    if s.len() == 1 {
        return SnailNumber {value: Some(s.parse().unwrap()), left: None, right:None};
    }
    let subs = &s[1..s.len()-1];
    let (pos,_) = subs.chars()
        .scan((0,0, '_'), |state, x| { // for each char: keep track of open and closed brackets up to this char
            state.0 += (x=='[') as u32;
            state.1 += (x==']') as u32;
            state.2 = x;
            Some(*state)
        })
        .enumerate()
        .filter(|(_,s)| s.2 == ',') // filter for comma-positions
        .map(|(i,state)| (i, ((state.0-state.1) as usize, state.2))) // calculate difference of open- and close brackets at each comma
        .min_by_key(|v| v.1.0) // get comma with minimum distance
        .unwrap(); 

    // this comma can be used to split the string into both halves
    
    let left = &subs[0..pos];
    let right = &subs[pos+1..];

    return SnailNumber {value: None, left: Some(Box::new(build(left))), right: Some(Box::new(build(right)))};
}


impl FromStr for SnailNumber {
    type Err = ();
    fn from_str(s: &str) -> Result<SnailNumber,()> {
        Ok(build(s))
    }
} 

impl Add for SnailNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        SnailNumber {
            value: None,
            left: Some(Box::new(self)),
            right: Some(Box::new(other))
        }
    }
}



pub fn first(content: &str) -> u32 {
    let numbers = content.split("\n").map(|x| x.parse::<SnailNumber>().unwrap()).collect::<Vec<SnailNumber>>();


    let nn = numbers[0].clone() + numbers[1].clone();

    
    println!("{}", nn.print());
    println!("{}", reduce(nn).print());

    // println!("{}", explode(nn.clone(),0).print());
    // println!("{}", split(nn.clone()).print());
    0
}


pub fn second(content: &str) -> u32 {
    0
}