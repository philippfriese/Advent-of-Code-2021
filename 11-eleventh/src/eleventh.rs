pub fn first(content: &str) -> usize {
    const SIZE: usize = 10;
    let mut data = [[0;SIZE+2];SIZE+2];
    
    content
        .split("\n")
        .enumerate().for_each(|(i,row)| {
            let mut char_iter = row.chars();
            let mut datum = [0;SIZE+2];
            for j in 1..SIZE+1 { datum[j] = char_iter.next().unwrap().to_digit(10).unwrap(); }
            data[i+1] = datum;
        });

    let mut flashes = 0;

    for _ in 0..100 {
        // first: increase energy levels
        data.iter_mut().for_each(|r| r.iter_mut().for_each(|el| *el+=1));
        

        // second: flash
        let mut changed = true;
        let mut flashers = Vec::new();
        while changed {
            changed = false;

            for i in 1..SIZE+1 {
                for j in 1..SIZE+1 {
                    if data[i][j] > 9 && !flashers.contains(&(i,j)) {
                        changed = true;

                        flashers.push((i,j));

                        data[i-1][j] += 1;
                        data[i+1][j] += 1;

                        data[i][j-1] += 1;
                        data[i][j+1] += 1;

                        data[i-1][j-1] += 1;
                        data[i+1][j+1] += 1;

                        data[i+1][j-1] += 1;
                        data[i-1][j+1] += 1;
                    }
                }
            }
        }

        // finally: reset flashers
        flashes += flashers.len();
        flashers.iter().for_each(|(i,j)| data[*i][*j] = 0);
    }

    flashes
}

pub fn second(content: &str) -> u64 {
    const SIZE: usize = 10;
    let mut data = [[0;SIZE+2];SIZE+2];
    
    content
        .split("\n")
        .enumerate().for_each(|(i,row)| {
            let mut char_iter = row.chars();
            let mut datum = [0;SIZE+2];
            for j in 1..SIZE+1 { datum[j] = char_iter.next().unwrap().to_digit(10).unwrap(); }
            data[i+1] = datum;
        });

    let mut flash_step = 0;

    let mut run = true;
    while run  {
        flash_step += 1;
        // first: increase energy levels
        data.iter_mut().for_each(|r| r.iter_mut().for_each(|el| *el+=1));
        

        // second: flash
        let mut changed = true;
        let mut flashers = Vec::new();
        while changed {
            changed = false;

            for i in 1..SIZE+1 {
                for j in 1..SIZE+1 {
                    if data[i][j] > 9 && !flashers.contains(&(i,j)) {
                        changed = true;

                        flashers.push((i,j));

                        data[i-1][j] += 1;
                        data[i+1][j] += 1;

                        data[i][j-1] += 1;
                        data[i][j+1] += 1;

                        data[i-1][j-1] += 1;
                        data[i+1][j+1] += 1;

                        data[i+1][j-1] += 1;
                        data[i-1][j+1] += 1;
                    }
                }
            }
        }

        run = flashers.len() != SIZE*SIZE;
        
        // finally: reset flashers
        flashers.iter().for_each(|(i,j)| data[*i][*j] = 0);
    }

    flash_step
}