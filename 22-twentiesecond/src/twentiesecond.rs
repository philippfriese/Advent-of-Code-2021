use regex::Regex;
use std::cmp::{min,max};
struct Command {
    x: (i32,i32),
    y: (i32,i32),
    z: (i32,i32),
    value: bool
}

pub fn first(content: &str) -> u32 {
    const SINGLE: i32 = 50;
    const SIZE: usize = 2*50+1;
    let re = Regex::new(r"^(on|off)\sx=((-?\d+)\.\.(-?\d+)),y=((-?\d+)\.\.(-?\d+)),z=((-?\d+)\.\.(-?\d+))$")
        .unwrap();
    
    let mut cube = vec![0;SIZE.pow(3)];

    let commands = content
        .split("\n")
        .map(|x| {
            let c = re.captures(x).unwrap();
            Command {
                value:c.get(1).unwrap().as_str() == "on",
                x: (c.get( 3).unwrap().as_str().parse::<i32>().unwrap(),
                    c.get( 4).unwrap().as_str().parse::<i32>().unwrap()),
                y: (c.get( 6).unwrap().as_str().parse::<i32>().unwrap(),
                    c.get( 7).unwrap().as_str().parse::<i32>().unwrap()),
                z: (c.get( 9).unwrap().as_str().parse::<i32>().unwrap(),
                    c.get(10).unwrap().as_str().parse::<i32>().unwrap()),
            }
        });

    commands
        .filter(|c| c.x.0 >= -SINGLE && c.x.1 <= SINGLE)
        .for_each(|c|{
        for x in c.x.0..=c.x.1 {
            for y in c.y.0..=c.y.1 {
                for z in c.z.0..=c.z.1 {
                    cube[(z+SINGLE) as usize +
                         (y+SINGLE) as usize *SIZE +
                         (x+SINGLE) as usize *SIZE*SIZE] = c.value as u8;
                }
            }   
        }
    });

    let mut x: u32 = 0;
    cube.iter().for_each(|v| x += *v as u32); // for whatever reason, rust doesn't want to fold into a u32, just into u16; this is too short

    x
    
}

#[derive(Copy,Clone,Debug)]
struct Cuboid {
    x1: i32, y1: i32, z1: i32, 
    x2: i32, y2: i32, z2: i32,
    value: bool
}

impl Cuboid {
    #[inline]
    fn get_area(self) -> i64 {
        (self.x2-self.x1) as i64 *
        (self.y2-self.y1) as i64 * 
        (self.z2-self.z1) as i64
    }



    // inspired by https://github.com/juanplopes
    // this essentially extracts the six possible cuboids from two intersecting? cuboids
    // 
    fn subtract(self, b: &Cuboid, value: bool) -> Vec<Cuboid> {
        let a = self; // hack; too tired to rename
        if ! 
            (   a.x1 < b.x2 && a.x2 > b.x1
             && a.y1 < b.y2 && a.y2 > b.y1
             && a.z1 < b.z2 && a.z2 > b.z1 ) {
                return vec![a.clone()];
            }
        else {
            let m = Cuboid {
                x1:min(max(b.x1, a.x1), a.x2), x2:min(max(b.x2, a.x1), a.x2),
                y1:min(max(b.y1, a.y1), a.y2), y2:min(max(b.y2, a.y1), a.y2),
                z1:min(max(b.z1, a.z1), a.z2), z2:min(max(b.z2, a.z1), a.z2),

                value: value};


            return vec![
                Cuboid {x1:a.x1, x2:m.x1, y1:a.y1, y2:a.y2, z1:a.z1, z2:a.z2, value:value },
                Cuboid {x1:m.x2, x2:a.x2, y1:a.y1, y2:a.y2, z1:a.z1, z2:a.z2, value:value },
                Cuboid {x1:m.x1, x2:m.x2, y1:a.y1, y2:m.y1, z1:a.z1, z2:a.z2, value:value },
                Cuboid {x1:m.x1, x2:m.x2, y1:m.y2, y2:a.y2, z1:a.z1, z2:a.z2, value:value },
                Cuboid {x1:m.x1, x2:m.x2, y1:m.y1, y2:m.y2, z1:a.z1, z2:m.z1, value:value },
                Cuboid {x1:m.x1, x2:m.x2, y1:m.y1, y2:m.y2, z1:m.z2, z2:a.z2, value:value },

             ];
        }
    }

}
pub fn second(content: &str) -> i64 {
    let re = Regex::new(r"^(on|off)\sx=((-?\d+)\.\.(-?\d+)),y=((-?\d+)\.\.(-?\d+)),z=((-?\d+)\.\.(-?\d+))$")
        .unwrap();
    

    let cuboids = content
        .split("\n")
        .map(|x| {
            let c = re.captures(x).unwrap();
            Cuboid {
                value:c.get(1).unwrap().as_str() == "on",

                x1: c.get( 3).unwrap().as_str().parse::<i32>().unwrap(),
                y1: c.get( 6).unwrap().as_str().parse::<i32>().unwrap(),
                z1: c.get( 9).unwrap().as_str().parse::<i32>().unwrap(),

                x2: c.get( 4).unwrap().as_str().parse::<i32>().unwrap()+1,
                y2: c.get( 7).unwrap().as_str().parse::<i32>().unwrap()+1,
                z2: c.get(10).unwrap().as_str().parse::<i32>().unwrap()+1,
            }
        });


    let mut newcuboids: Vec<Cuboid> = Vec::new();
    for c in cuboids {
        let mut temp: Vec<Cuboid> = Vec::new();
        for other in &newcuboids {
            let mut sub = other.subtract(&c, false)
                .iter()
                .filter(|x| x.get_area() > 0)
                .copied()
                .collect::<Vec<_>>();
            temp.append(&mut sub);
        }
        newcuboids = temp;

        if c.value {
             newcuboids.push(c);
        }
    }
    newcuboids.iter().map(|x| x.get_area()).sum()
    
}
