use petgraph::graph::DiGraph;

use petgraph::graph::NodeIndex;


#[derive(Debug,Eq)]
struct Node<'a> {
    name: &'a str,
    big: bool
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
         self.name == other.name
    }
}


// this all is a recursive nightmare

pub fn first(content: &str) -> usize {
    let mut node_map = std::collections::HashMap::new();
    let mut graph = DiGraph::<Node, bool>::new();
    content
        .split("\n")
        .for_each(|line| {
            let mut split = line.split("-");
            let lhs_id = split.next().unwrap(); 
            let lhs_big = lhs_id.chars().fold(true, |acc,r| acc && r.is_ascii_uppercase());
            let rhs_id = split.next().unwrap();
            let rhs_big = rhs_id.chars().fold(true, |acc,r| acc && r.is_ascii_uppercase());


            if !node_map.contains_key(lhs_id) {
                node_map.insert(lhs_id, graph.add_node(Node{name:lhs_id, big:lhs_big}));
            }
            if !node_map.contains_key(rhs_id){
                node_map.insert(rhs_id, graph.add_node(Node{name:rhs_id, big:rhs_big}));
            }
 
            if rhs_id != "start" && lhs_id != "end" {
                graph.add_edge(*node_map.get(lhs_id).unwrap(), *node_map.get(rhs_id).unwrap(), !rhs_big);
            }

            if lhs_id != "start" && rhs_id != "end" {
                graph.add_edge(*node_map.get(rhs_id).unwrap(), *node_map.get(lhs_id).unwrap(), !lhs_big);
            }
              
        });



    let start_node = node_map.get("start").unwrap();

    let mut paths = Vec::new();
    let init_path = Vec::from([graph.node_weight(*start_node).unwrap()]);

    find_paths_first(&graph, &mut paths, init_path, start_node);
    paths.len()
}

fn find_paths_first(graph: &DiGraph<Node,bool>, paths: &mut Vec<Vec<String>>, path: Vec<&Node>, node: &NodeIndex) {
    if graph.node_weight(*node).unwrap().name == "end" {
        paths.push(path.iter().map(|x| x.name.to_string()).collect()); 
    }
    for n in graph.neighbors(*node) {
        let to = graph.node_weight(n).unwrap();

        if to.big || !path.contains(&to) { 
          let mut new_path = path.clone();
          new_path.push(&to);
          find_paths_first(graph, paths, new_path, &n);  
        }
    }
}

pub fn second(content: &str) -> usize {
     let mut node_map = std::collections::HashMap::new();
    let mut graph = DiGraph::<Node, bool>::new();
    content
        .split("\n")
        .for_each(|line| {
            let mut split = line.split("-");
            let lhs_id = split.next().unwrap(); 
            let lhs_big = lhs_id.chars().fold(true, |acc,r| acc && r.is_ascii_uppercase());
            let rhs_id = split.next().unwrap();
            let rhs_big = rhs_id.chars().fold(true, |acc,r| acc && r.is_ascii_uppercase());


            if !node_map.contains_key(lhs_id) {
                node_map.insert(lhs_id, graph.add_node(Node{name:lhs_id, big:lhs_big}));
            }
            if !node_map.contains_key(rhs_id){
                node_map.insert(rhs_id, graph.add_node(Node{name:rhs_id, big:rhs_big}));
            }
 
            if rhs_id != "start" && lhs_id != "end" {
                graph.add_edge(*node_map.get(lhs_id).unwrap(), *node_map.get(rhs_id).unwrap(), !rhs_big);
            }

            if lhs_id != "start" && rhs_id != "end" {
                graph.add_edge(*node_map.get(rhs_id).unwrap(), *node_map.get(lhs_id).unwrap(), !lhs_big);
            }
              
        });



    let start_node = node_map.get("start").unwrap();
    let mut num_traversed = std::collections::HashMap::new();
    for node in node_map.keys() {
        num_traversed.insert(String::from(*node), 0);
    }
    let mut paths = Vec::new();
    let init_path = Vec::from([graph.node_weight(*start_node).unwrap()]);

    find_paths_second(&graph, &mut paths, init_path, start_node, &mut num_traversed);
    // paths.iter().for_each(|p| println!("{:?}", p));
    paths.len()
}

fn find_paths_second(graph: &DiGraph<Node,bool>, paths: &mut Vec<Vec<String>>, path: Vec<&Node>, node: &NodeIndex, 
    num_traversed: &mut std::collections::HashMap<String,usize>) {
    if graph.node_weight(*node).unwrap().name == "end" {
        paths.push(path.iter().map(|x| x.name.to_string()).collect()); 
    }   
    for n in graph.neighbors(*node) {
        let to = graph.node_weight(n).unwrap();
  
        if ((*num_traversed.get(&String::from(to.name)).unwrap() < 2) && !to.big && num_traversed.values().filter(|x| **x >= 2).collect::<Vec<&usize>>().len() < 2) || to.big { 
            let mut new_num_traversed = num_traversed.clone();
            if !to.big {
                 if let Some(v) = new_num_traversed.get_mut(&String::from(to.name)) { *v+=1;}
            }
           
            let mut new_path = path.clone();
            new_path.push(&to);
            find_paths_second(graph, paths, new_path, &n, &mut new_num_traversed);  
        }
    }
}