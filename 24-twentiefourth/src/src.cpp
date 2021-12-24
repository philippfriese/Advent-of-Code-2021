
fn single_run(inp: i64, run: (i64,i64,i64), registers: (i64,i64,i64,i64)) -> (i64,i64,i64,i64) {
    let mut x = registers.0;
    let mut y = registers.1;
    let mut z = registers.2;
    let mut w = registers.3;

    w = inp;
    x = 0;
    x = x + z;
    x = x % 26;

    z = z / run.0;
    x = x + run.1;
    x = (x == w) as i64;

    x = (x == 0) as i64;
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

pub fn first(_content: &str) -> u16 {
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

    let input = [1,2,3,4,5,6,7,8,9,10,11,12,13,14];
    let mut registers = (0,0,0,0);
    for (run, inp) in runs.iter().zip(input.iter()) {
        registers = single_run(*inp, *run, registers);
    }
    println!("{:?}", registers);
    0
}



pub fn second(content: &str) -> i128 {
    0
}
