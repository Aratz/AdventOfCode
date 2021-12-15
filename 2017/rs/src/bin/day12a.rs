extern crate regex;

use regex::Regex;
use std::io::{self, Read};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let re = Regex::new(r"(?P<parent>\d+) <-> (?P<children>(\d+, )*\d+)").unwrap();

    let mut buffer = String::new();

    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    for program in re.captures_iter(&buffer) {
        let parent = program.name("parent").unwrap().as_str().parse().unwrap();
        for child in program.name("children").unwrap().as_str().split(", ") {
            let child = child.parse().unwrap();
            graph.entry(parent).or_default().insert(child);
            graph.entry(child).or_default().insert(parent);
        }
    }

    let mut zero_group: HashSet<i32> = HashSet::new();
    let mut queue: VecDeque<i32> = VecDeque::new();

    queue.push_back(0);

    while let Some(program) = queue.pop_front() {
        if !zero_group.contains(&program) {
            zero_group.insert(program);
            for child in graph[&program].iter().filter(|p| !zero_group.contains(p)) {
                queue.push_back(*child)
            }
        }
    }

    println!("{}", zero_group.len());

}
