
use std::collections::HashMap;
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