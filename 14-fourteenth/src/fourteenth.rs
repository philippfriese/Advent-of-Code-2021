
use std::collections::HashMap;
use itertools::{Itertools, enumerate};

#[derive(Debug)]
struct Rule {
    lhs: char,
    rhs: char,
    tar: char
}



pub fn first(content: &str, num_steps: usize) -> usize {
    let mut content_split = content.split("\n");
    let mut template = content_split.next().unwrap().chars().collect::<Vec<char>>();

    content_split.next();

    let rules: Vec<Rule> = content_split.map(|x| {
        let mut rule_split = x.split(" -> ");
        let mut sources = rule_split.next().unwrap().chars();
        let target = rule_split.next().unwrap().chars().next().unwrap();
        Rule {lhs: sources.next().unwrap(), rhs: sources.next().unwrap(), tar: target}
    })
    .collect();

    let mut insertions = Vec::new();
    for _ in 0..num_steps {
        for i in 1..template.len() {
            let lhs = template[i-1];
            let rhs = template[i];
            for rule in &rules {
                if rhs == rule.rhs && lhs == rule.lhs {
                    insertions.push((i,rule.tar));
                }
            }
        }
        insertions.iter().enumerate().for_each(|(offset,(pos,c))| template.insert(pos+offset,*c));
        insertions.clear();
    }
    
    let mut counter = vec![0;26];
    template.iter().for_each(|x| counter[*x as usize - 'A' as usize] += 1);

    let max = counter.iter().filter(|x|**x>0).max().unwrap();
    let min = counter.iter().filter(|x|**x>0).min().unwrap();
 
    max-min
}

pub fn second(content: &str, num_steps: usize) -> usize {
    let mut content_split = content.split("\n");
    let template = content_split.next().unwrap().chars().collect::<Vec<char>>();

    content_split.next();

    let rules: Vec<Rule> = content_split
        .map(|x| {
            let mut rule_split = x.split(" -> ");
            let mut sources = rule_split.next().unwrap().chars();
            let target = rule_split.next().unwrap().chars().next().unwrap();
            Rule {lhs: sources.next().unwrap(), rhs: sources.next().unwrap(), tar: target}
        })
        .collect();

    let mut counters = HashMap::new();
    
    rules.iter().for_each(|r| {counters.insert((r.lhs, r.rhs), 0);});
   
    for i in 1..template.len() {
        if let Some(v) = counters.get_mut(&(template[i-1],template[i])) { *v += 1; }
    }
    
    let mut g_counter = vec![0;26];
    template.iter().for_each(|x| g_counter[*x as usize - 'A' as usize] += 1);

    for _ in 0..num_steps {
        let mut counters_temp = counters.clone();

        for rule in &rules {
            for ((lhs,rhs),c) in &counters {
                if *c == 0 { continue; }
            
                if *rhs == rule.rhs && *lhs == rule.lhs {
                    // rule matches, so the from tuple is destroyed
                    if let Some(v) = counters_temp.get_mut(&(*lhs,*rhs)) {
                        *v -= c;
                    }

                    // add newly created tuples
                    if let Some(v) = counters_temp.get_mut(&(*lhs,rule.tar)) {
                        *v += c;
                        g_counter[rule.tar as usize - 'A' as usize] += c; // only count rule.tar once
                    }
                    if let Some(v) = counters_temp.get_mut(&(rule.tar,*rhs)) {
                        *v += c;
                    }
                }
            }
        }
        counters = counters_temp;
    }

    let mut counter = vec![0;26];
    counters.iter().for_each(|((lhs,rhs), c)| {
        counter[*lhs as usize - 'A' as usize] += c;
        counter[*rhs as usize - 'A' as usize] += c;
    });

    let max = g_counter.iter().filter(|x|**x>0).max().unwrap();
    let min = g_counter.iter().filter(|x|**x>0).min().unwrap();
 
    max-min
}



pub fn run_01(content: &str)  -> i32{
    let parts = content
        .split("\n\n")
        .collect::<Vec<_>>();

    let initial_pattern = String::from(parts[0]);

    let mut instructions: HashMap<String, char> = HashMap::new();
    parts[1]
        .split("\n")
        .map(|x| x.split(" -> ").collect::<Vec<_>>())
        .for_each(|x| {
            instructions.insert(String::from(x[0]), *x[1].chars().collect::<Vec<_>>().first().unwrap());
        });

    let mut current_pattern = initial_pattern;

    // println!("Initial pattern: {}", current_pattern);

    for round in 0..10 {
        let mut insertions: Vec<(usize, char)> = vec![];

        for (index, window) in (current_pattern.chars().collect::<Vec<_>>().windows(2)).enumerate() {
            let string_window = String::from_iter(window.iter());
            let instruction = instructions.get(&string_window);
            match instruction {
                Some(value) => {
                    insertions.push((index + 1, value.clone()))
                },
                _ => continue,
            };
        }
        // println!("{:?}", insertions);

        for (current_index, (insertion_index, insertion_char)) in insertions.iter().enumerate() {
            current_pattern.insert(insertion_index + current_index, *insertion_char)
        }
        // println!("Round: {}, pattern: {}", round + 1, current_pattern);
    }

    let unique_letters = current_pattern
        .chars()
        .into_iter()
        .unique()
        .collect::<Vec<_>>();


    let mut letter_occ = unique_letters
        .iter()
        .map(|x| (*x, current_pattern.chars().into_iter().filter(|y| *y == *x).count()))
        .collect::<Vec<_>>();

    letter_occ.sort_by_key(|x| x.1);

    let min = letter_occ.first().unwrap().1;
    let max = letter_occ.last().unwrap().1;

    // println!("Letter occ: {:?}", letter_occ);
    max as i32 - min as i32
}


/**
== Initial ==== Second ====
CH = 0          CH = 1
CB = 1          CB = 0
HH = 0          HH = 0
NH = 0          NH = 0
HB = 0          HB = 1
HC = 0          HC = 0
HN = 0          HN = 0
NN = 1          NN = 0
BH = 0          BH = 0
NC = 1          NC = 1
NB = 0          NB = 1
BN = 0          BN = 0
BB = 0          BB = 0
BC = 0          BC = 1
CC = 0          CC = 0
CN = 0          CN = 1
C = 2           C = 4       = 2
B = 2           B = 3 + 1   = 2
N = 4           N = 3 + 1   = 2
                H = 2       = 1
**/
pub fn run_02(content: &str) -> u64{
    let parts = content
        .split("\n\n")
        .collect::<Vec<_>>();

    let initial_pattern = String::from(parts[0]);

    let mut occ_counter :HashMap<String, u64> = HashMap::new();
    let mut instructions: HashMap<String, (String, String)> = HashMap::new();
    parts[1]
        .split("\n")
        .map(|x| x.split(" -> ").collect::<Vec<_>>())
        .for_each(|x| {
            let pair_1 = format!("{}{}", x[0].chars().collect::<Vec<_>>().first().unwrap(), x[1].chars().collect::<Vec<_>>().first().unwrap());
            let pair_2 = format!("{}{}", x[1].chars().collect::<Vec<_>>().first().unwrap(), x[0].chars().collect::<Vec<_>>().last().unwrap());
            instructions.insert(String::from(x[0]), (pair_1, pair_2));
            occ_counter.insert(String::from(x[0]), 0);
        });

    initial_pattern
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|x| {
            let string_window = String::from_iter(x.iter());
            *occ_counter.get_mut(&string_window).unwrap() += 1;
        });
    //println!("Start {:?}", calc_letters(&occ_counter));
    for round in 0..40 {
        occ_counter = iteration(&occ_counter, &instructions);
        // println!("Round: {} {:?}",round + 1, calc_letters(&occ_counter));
    }

    let mut unique_letters = calc_letters(&occ_counter);
    unique_letters.sort_by_key(|x| x.1);

    let min = unique_letters.first().unwrap().1;
    let max = unique_letters.last().unwrap().1;

    // println!("Letter occ: {:?}", letter_occ);
    max as u64 - min as u64
}

fn iteration(current_pairs: &HashMap<String, u64>, instructions: &HashMap<String, (String, String)>) -> HashMap<String, u64> {
    let mut return_hash_map : HashMap<String, u64> = HashMap::new();
    for (key, value) in current_pairs {
        let (key_1, key_2) = instructions.get(key).unwrap();
        *return_hash_map.entry(key_1.clone()).or_insert(0) += value;
        *return_hash_map.entry(key_2.clone()).or_insert(0) += value;
    }
    return return_hash_map;
}

fn calc_letters(pairs: &HashMap<String, u64>) -> Vec<(char, u64)> {
    let mut return_hash_map: HashMap<char, u64> = HashMap::new();
    for key in pairs.keys() {
        let value = *pairs.get(key).unwrap();
        let chars = key.chars().collect::<Vec<_>>();
        *return_hash_map.entry(chars[0]).or_insert(0) += value;
        *return_hash_map.entry(chars[1]).or_insert(0) += value;
    }

    return return_hash_map
        .into_iter()
        .map(|(x, y)| return if y % 2 == 0{
            (x, (y / 2) as u64)
        } else {
            (x, ((y + 1) / 2) as u64)
        })
        .collect::<Vec<_>>()
}