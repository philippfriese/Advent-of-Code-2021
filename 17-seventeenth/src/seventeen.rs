pub fn first(content: &str) -> i16 {
    let split = &mut content[13..].split(",");
    let mut y = split.nth(1).unwrap().split("..");
    let ymin = y.next().unwrap()[3..].parse::<i16>().unwrap();

    (ymin.abs() * (ymin.abs()-1))/2
}

fn step(xbounds: (i32,i32), ybounds: (i32,i32), mut v: (i32,i32)) -> bool {
    let mut pos = (0,0);

    loop {
        pos.0 += v.0;
        pos.1 += v.1;

        v.0 = std::cmp::max(v.0-1,0);
        v.1 -= 1;

        if xbounds.0 <= pos.0 && pos.0 <= xbounds.1 
        && ybounds.0 <= pos.1 && pos.1 <= ybounds.1 { return true; } 
        else if pos.1 < ybounds.0 || pos.0 > xbounds.1 { return false; }

    }
}

pub fn second(content: &str) -> i32 {
    let split = &mut content[13..].split(",");
    let mut x = split.next().unwrap().split("..");
    let xmin = x.next().unwrap()[2..].parse::<i16>().unwrap();
    let xmax = x.next().unwrap().parse::<i16>().unwrap();
    let mut y = split.next().unwrap().split("..");
    let ymin = y.next().unwrap()[3..].parse::<i16>().unwrap();
    let ymax = y.next().unwrap().parse::<i16>().unwrap();

    let xbound = (xmin as i32, xmax as i32);
    let ybound = (ymin as i32,ymax as i32);

    let vymax = (ymin.abs()-1) as i32;
    let vymin = ymin as i32;

    let vxmax = xmax as i32;
    let vxmin = (((2*xmin) as f32).sqrt().ceil() -1.) as i32;

    let mut c = 0;
    for vx in vxmin..=vxmax {
        for vy in vymin..=vymax {
            c += step(xbound, ybound, (vx,vy)) as i32;
        }
    }
    c
}