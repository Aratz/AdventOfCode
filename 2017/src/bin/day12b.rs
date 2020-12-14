extern crate regex;

use regex::Regex;
use std::io::{self, Read};
use std::collections::{HashMap, HashSet};


fn union(i: usize, j:usize, mut groups: &mut Vec<usize>) {
    let leader_i = find(i, &mut groups);
    let leader_j = find(j, &mut groups);
    groups[leader_j] = leader_i;
}

fn find(i: usize, mut groups: &mut Vec<usize>) -> usize {
    if groups[i] != i {
        groups[i] = find(groups[i], &mut groups);
    }

    groups[i]
}

fn main() {
    let re = Regex::new(r"(?P<parent>\d+) <-> (?P<children>(\d+, )*\d+)").unwrap();

    let mut buffer = String::new();

    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();

    for program in re.captures_iter(&buffer) {
        let parent = program.name("parent").unwrap().as_str().parse().unwrap();
        for child in program.name("children").unwrap().as_str().split(", ") {
            let child = child.parse().unwrap();
            graph.entry(parent).or_default().insert(child);
            graph.entry(child).or_default().insert(parent);
        }
    }

    let mut groups: Vec<usize> = (0..graph.len()).collect();

    for (&parent, children) in graph.iter() {
        for &child in children.iter() {
            union(parent, child, &mut groups);
        }
    }

    for i in 0..groups.len() {
        find(i, &mut groups); // Make sure the tree is in compact form
    }

    println!("{:?}", groups.clone().iter().map(|&i| find(i, &mut groups)).collect::<HashSet<_>>().len());
}
