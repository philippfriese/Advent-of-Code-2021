use std::collections::HashMap;

fn single_run(run: (i64,i64,i64), z: i64, w: i64) -> i64 {

    let mut newz = z / run.0;

    if (z % 26) + run.1 != w {
        newz = newz*26 + w + run.2;
    } 

    newz
}


fn run(inp: Vec::<i64>) -> i64 {

    let mut carry = 0;
    carry =    (carry /  1) * (25 * (((carry % 26) +  15) != inp[ 0]) as i64 + 1) + (inp[ 0] + 13) * (((carry % 26) +  15) != inp[ 0]) as i64;
    carry =    (carry /  1) * (25 * (((carry % 26) +  10) != inp[ 1]) as i64 + 1) + (inp[ 1] + 16) * (((carry % 26) +  10) != inp[ 1]) as i64;
    carry =    (carry /  1) * (25 * (((carry % 26) +  12) != inp[ 2]) as i64 + 1) + (inp[ 2] +  2) * (((carry % 26) +  12) != inp[ 2]) as i64;
    carry =    (carry /  1) * (25 * (((carry % 26) +  10) != inp[ 3]) as i64 + 1) + (inp[ 3] +  8) * (((carry % 26) +  10) != inp[ 3]) as i64;
    carry =    (carry /  1) * (25 * (((carry % 26) +  14) != inp[ 4]) as i64 + 1) + (inp[ 4] + 11) * (((carry % 26) +  14) != inp[ 4]) as i64;
    carry =    (carry / 26) * (25 * (((carry % 26) + -11) != inp[ 5]) as i64 + 1) + (inp[ 5] +  6) * (((carry % 26) + -11) != inp[ 5]) as i64;
    carry =    (carry /  1) * (25 * (((carry % 26) +  10) != inp[ 6]) as i64 + 1) + (inp[ 6] + 12) * (((carry % 26) +  10) != inp[ 6]) as i64;
    carry =    (carry / 26) * (25 * (((carry % 26) + -16) != inp[ 7]) as i64 + 1) + (inp[ 7] +  2) * (((carry % 26) + -16) != inp[ 7]) as i64;
    carry =    (carry / 26) * (25 * (((carry % 26) +  -9) != inp[ 8]) as i64 + 1) + (inp[ 8] +  2) * (((carry % 26) +  -9) != inp[ 8]) as i64;
    carry =    (carry /  1) * (25 * (((carry % 26) +  11) != inp[ 9]) as i64 + 1) + (inp[ 9] + 15) * (((carry % 26) +  11) != inp[ 9]) as i64;
    carry =    (carry / 26) * (25 * (((carry % 26) +  -8) != inp[10]) as i64 + 1) + (inp[10] +  1) * (((carry % 26) +  -8) != inp[10]) as i64;
    carry =    (carry / 26) * (25 * (((carry % 26) +  -8) != inp[11]) as i64 + 1) + (inp[11] + 10) * (((carry % 26) +  -8) != inp[11]) as i64;
    carry =    (carry / 26) * (25 * (((carry % 26) + -10) != inp[12]) as i64 + 1) + (inp[12] + 14) * (((carry % 26) + -10) != inp[12]) as i64;
    carry =    (carry / 26) * (25 * (((carry % 26) +  -9) != inp[13]) as i64 + 1) + (inp[13] + 10) * (((carry % 26) +  -9) != inp[13]) as i64;
  
    carry 
}

fn single_run_2(inp: i128, run: (i128,i128,i128), registers: (i128,i128,i128,i128)) -> (i128,i128,i128,i128) {
    let mut x = registers.0;
    let mut y = registers.1;
    let mut z = registers.2;
    let mut w = registers.3;
    w = inp;
    x = 0;
    x = x + z;
    x = x % 26;

    // z = (z as f64 / run.0 as f64).floor() as i128;
    z = z / run.0;
    
    x = x + run.1;
    x = (x == w) as i128;
    x = (x == 0) as i128;

    y = 0;
    y = y + 25;

    y = y * x;
    y = y + 1;

    z = z * y;

    y = 0;
    y = y + w;
    y = y + run.2;

    y = y * x;
    
    z = z + y;

    (x,y,z,w)
}


// taken from u/i_have_no_biscuits
pub fn doit() -> HashMap<i64, (i64,i64)> {
    let runs = [
        ( 1,  15, 13),
        ( 1,  10, 16),
        ( 1,  12,  2),
        ( 1,  10,  8),
        ( 1,  14, 11),
        (26, -11,  6),
        ( 1,  10, 12),
        (26, -16,  2),
        (26,  -9,  2),
        ( 1,  11, 15),
        (26,  -8,  1),
        (26,  -8, 10),
        (26, -10, 14),
        (26,  -9, 10),
    ];

    let mut zs = HashMap::new();
    zs.insert(0,(0,0));

    // idea: each run/ parameter set can be used to determine the min_max bound of the associated digit
    // this is possible as we go from left to right; all subsequent runs will impact a smaller part of the number

    for (i,run) in runs.iter().enumerate() {
        let mut newzs =HashMap::new();

        // for all currently found z-values (carry values), search for a new, better z value
        for (z,inp) in zs.iter() {
            for w in 1..=9 {
                let newz = single_run(*run, *z, w);

                // only do work if the current param set divides by 1 (aka the number gets bigger/ doesn't get divided)
                // or if the number gets divided AND the newly found z is smaller
                if run.0 == 1 || (run.0 == 26 && newz < *z) {
                    newzs.insert(newz, (
                        // keep track of both the min and max value of the current z value;
                        // add the current digit to it by "shifting" the current input value to the left by 1 digit (aka mult by 10)
                        std::cmp::min(newzs.get(&newz).unwrap_or(&(inp.0*10+w,0)).0, inp.0*10+w), 
                        std::cmp::max(newzs.get(&newz).unwrap_or(&(0,inp.1*10+w)).1, inp.1*10+w)  
                    ));
                }
            } 
        }

        zs = newzs;

    }
    zs
}


pub fn first(_content: &str) -> i64 {
    let zs = doit();
    let (_,upper) = zs.get(&0).unwrap();
    *upper
}



pub fn second(_content: &str) -> i64 {
    let zs = doit();
    let (lower,_) = zs.get(&0).unwrap();
    *lower
    
}
