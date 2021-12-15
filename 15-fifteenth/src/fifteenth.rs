use petgraph::graph::DiGraph;
use petgraph::algo::dijkstra;

#[derive(Debug,Default,Clone,Copy)]
struct Node {
    x: usize,
    y: usize,
    weight: u32
}

// this is all slooowwwww
pub fn first(content: &str, size: usize) -> u32 {
    let mut node_map = std::collections::HashMap::new();
    let mut graph = DiGraph::<_, u32>::new();
    content.split("\n").enumerate().for_each(|(row_idx, row)| {
            row.chars().enumerate().for_each(|(col_idx, c)| {
                let weight = c.to_digit(10).unwrap();
                let cur = Node {x:row_idx, y:col_idx, weight:weight};
                node_map.insert((row_idx,col_idx), graph.add_node(cur));      
            });
              
        });

    for i in 0..size {
        for j in 0..size {
            let cur   = node_map.get(&(i,   j  )).unwrap();
            if i+1 < size {
                let to1   = node_map.get(&(i+1, j  )).unwrap();
                graph.add_edge(*cur,*to1,   graph.node_weight(*to1).unwrap().weight);
                graph.add_edge(*to1,*cur,   graph.node_weight(*cur).unwrap().weight);

            }
            if j+1 < size {
                let to2   = node_map.get(&(i,   j+1)).unwrap();
                
                graph.add_edge(*cur,*to2,   graph.node_weight(*to2).unwrap().weight);
                graph.add_edge(*to2,*cur,   graph.node_weight(*cur).unwrap().weight);
            }

        }
    }   
    let end = node_map.get(&(size-1,size-1)).unwrap();
    let path = dijkstra(
        &graph, 
        *node_map.get(&(0,0)).unwrap(),
        Some(*end),
        |e| *e.weight());
    *path.get(&end).unwrap() 
}

pub fn second(content: &str, size: usize) -> u32 {
    let mut node_map = std::collections::HashMap::new();
    let mut graph = DiGraph::<_, u32>::new();

    content.split("\n")
        .enumerate().for_each(|(row_idx, row)| {
            row.chars().enumerate().for_each(|(col_idx, c)| {
                for r_row in 0..5 {
                    for r_col in 0..5 {
                        node_map.insert(
                            (row_idx+size*r_row, col_idx+size*r_col), 
                            graph.add_node(Node {
                                x:row_idx+size*r_row, 
                                y:col_idx+size*r_col, 
                                weight:(((r_col+r_row) as u32+c.to_digit(10).unwrap())-1) % 9 + 1
                            }));
                }   
                }
            });
              
        });
    //  for i in 0..5*size {
    //     for j in 0..5*size {  
    //         let cur   = graph.node_weight(*node_map.get(&(i,   j  )).unwrap()).unwrap();
    //         print!("{}",cur.weight);
    //     }
    //     println!("");
    // }
    for i in 0..5*size {
        for j in 0..5*size {
            let cur   = node_map.get(&(i,   j  )).unwrap();

            if (i+1) < 5*size {
                let to1   = node_map.get(&(i+1, j  )).unwrap();
                graph.add_edge(*cur,*to1,   graph.node_weight(*to1).unwrap().weight);
                graph.add_edge(*to1,*cur,   graph.node_weight(*cur).unwrap().weight);
            }
            if (j+1) < 5*size {
                let to2   = node_map.get(&(i,   j+1)).unwrap();
                graph.add_edge(*cur,*to2,   graph.node_weight(*to2).unwrap().weight);
                graph.add_edge(*to2,*cur,   graph.node_weight(*cur).unwrap().weight);
            }
        }
    }   
   

    let end = node_map.get(&(5*size-1,5*size-1)).unwrap();
    let path = dijkstra(
        &graph, 
        *node_map.get(&(0,0)).unwrap(),
        Some(*end),
        |e| *e.weight());
    *path.get(&end).unwrap()
}