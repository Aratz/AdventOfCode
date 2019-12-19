extern crate itertools;

use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let mut lines = String::new();

    stdin.lock().read_to_string(&mut lines).unwrap();

    let lines = lines; // Make lines immutable again

    let direct_orbits: HashMap<&str, &str> = lines.lines()
        .flat_map(|l| l.split(')'))
        .tuples().flat_map(|(a, b)| vec![(b, a)]).collect();

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in lines.lines().map(|l| l.split(')').tuples().next().unwrap()) {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let (current_pos, stop) = (direct_orbits["YOU"], direct_orbits["SAN"]);

    let mut dist_to_you: HashMap<&str, i32> = HashMap::new();
    dist_to_you.entry(current_pos).or_insert(0);

    let mut queue = VecDeque::new();
    queue.push_back(current_pos);

    while let Some(p) = queue.pop_front() {
        let dist_p = dist_to_you[p];
        for dest in &graph[p] {
            if !dist_to_you.contains_key(dest) {
                queue.push_back(dest);
            }
            dist_to_you.entry(dest).or_insert({
                dist_p + 1
            });
        }
    }

    println!("{}", dist_to_you[stop]);
}
