pub fn first(content: &str, width: usize) -> u32 {
    let t = 
        content
            .split("\n")
            .map(|row| {
                ["9",row,"9"].concat().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>()
            });
    let data: Vec<Vec<u32>> = vec![vec![9;width+2];1]
        .into_iter().chain(t)
        .chain(vec![vec![9;width+2];1]).collect();
    
    let mut risk = 0;

    for row in 1..data.len()-1 {
        for col in 1..data[0].len()-1 {
        
            let center = data[row][col];

            let left = data[row][col-1]; 
            let right = data[row][col+1];  

            let up = data[row+1][col]; 
            let down = data[row-1][col]; 

            if center < left
            && center < right
            && center < up
            && center < down{
                risk += 1+center;
            }
        }
    }

    risk
}

pub fn second(content: &str, width: usize) -> i32 {
    // first: find lowpoints

    let t = 
        content
            .split("\n")
            .map(|row| {
                ["9",row,"9"].concat().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>()
            });
    let data: Vec<Vec<u32>> = vec![vec![9;width+2];1]
        .into_iter().chain(t)
        .chain(vec![vec![9;width+2];1]).collect();
    
    let mut lowpoints = Vec::new();

    for row in 1..data.len()-1 {
        for col in 1..data[0].len()-1 {
        
            let center = data[row][col];

            let left = data[row][col-1]; 
            let right = data[row][col+1];  

            let up = data[row+1][col]; 
            let down = data[row-1][col]; 

            if center < left
            && center < right
            && center < up
            && center < down{
                lowpoints.push((row,col));
            }
        }
    }


    // second: go through all lowpoints and recursively expand the basin
    let mut basins: Vec<Vec<(usize,usize)>> = Vec::new();
    for (p_row, p_col) in lowpoints {
        let mut basin = Vec::new();
        get_basins(&data,&mut basin, p_row, p_col);
        basins.push(basin);
    }

    let mut sizes = basins.iter().map(|x| x.len() as i32).collect::<Vec<i32>>();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()   
}

fn get_basins(data: &Vec<Vec<u32>>, acc: &mut Vec<(usize,usize)>, row: usize, col: usize){
    // recursively check for connected measurements in basin
    // note: this uses a vast amount of Vec.contains, which will not scale for large data
    //       however: we are dealing with a max of 100x100, so this should be fine
    //       fast it is not though
    if data[row][col] == 9 { return; }
    if !acc.contains(&(row,col)) {
        acc.push((row,col));
    }
    if 0 < row && !acc.contains(&(row-1,col))             { get_basins(data,acc, row-1,col); }
    if row < data.len() && !acc.contains(&(row+1,col))    { get_basins(data,acc, row+1,col); }

    if 0 < col && !acc.contains(&(row,col-1))             { get_basins(data,acc, row,col-1); }
    if col < data[0].len() && !acc.contains(&(row,col+1)) { get_basins(data,acc, row,col+1); }
}

pub fn second2(content: &str, width: usize, height: usize) -> i32 {
    // this does not work
    // idea: try to assign basins in O(width*height)
    // problem: in some situations, two "sides" of one basin are detected too early and assigned different basin IDs
    // potential solution: employ some form of merging strategy
    // however: as we are dealing with a 100x100 field, this is probably overkill
    let t = 
        content
            .split("\n")
            .map(|row| {
                ["9",row,"9"].concat().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>()
            });
    let data: Vec<Vec<u32>> = vec![vec![9;width+2];1]
        .into_iter().chain(t)
        .chain(vec![vec![9;width+2];1]).collect();
    
    

    let mut basin_ids: Vec<Vec<i32>> = vec![vec![-1;data[0].len()]; data.len()];

    let mut basins: Vec<Vec<u32>> = Vec::new();

    for row in 1..data.len()-1 {
        for col in 1..data[0].len()-1 {
        
            let center = data[row][col];
            if center == 9 { continue; }


            let left =            data[row][col-1]; 
            let basin_left = basin_ids[row][col-1]; 

            let right =            data[row][col+1]; 
            let basin_right = basin_ids[row][col+1]; 


            let up =            data[row+1][col]; 
            let basin_up = basin_ids[row+1][col]; 

            let down =            data[row-1][col]; 
            let basin_down = basin_ids[row-1][col]; 

            let mut basin_id = None; 
            for (dir,b_id) in [(center,basin_ids[row][col]), (left, basin_left), (right, basin_right), (up, basin_up), (down, basin_down)] {
                if dir != 9 && b_id != -1 {
                    // idea: if two ids are encountered: merge basins_arrays from both
                    // idea2: store unique array index and use hashset for merging
                    basin_id = match basin_id {
                        None => Some(b_id as usize),
                        Some(v) => Some(std::cmp::max(b_id as usize, v))
                    };
                    // basin_id = Some(b_id as usize);
                }
            }

            if basin_id == None { 
                basins.push(Vec::new());
                basin_id = Some(basins.len()-1); 
            }


            for (dir, _row, _col) in [(center,row,col), (left, row, col-1), (right, row, col+1), (up, row+1,col), (down, row-1,col)] {
                if dir != 9 {
                    // basins[basin_id.unwrap()].push(dir);
                    basin_ids[_row][_col] = basin_id.unwrap() as i32; 
                }
            } 

            if center != 9 {
                basins[basin_id.unwrap()].push(center);
            }

        }
    }

    for r in &basins { println!("{:?}", r); }
    for r in basin_ids { 
        for v in r {print!("{: >4} ",v); }
        println!("");
    }

    for r in data { 
        for v in r {print!("{: >2} ",v); }
        println!("");
    }
    let mut sizes = basins.iter().map(|x| x.len() as i32).collect::<Vec<i32>>();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
    
}