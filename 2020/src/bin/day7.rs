extern crate regex;

use std::io::{self, Read};
use regex::Regex;
use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re_rules = Regex::new(r"(?P<parent>\w+ \w+) bags contain(?P<children>( \d+ \w+ \w+ bags?(?:,|\.))*)").unwrap();
    let re_bags = Regex::new(r"(?P<n>\d+) (?P<color>\w+ \w+) bags?").unwrap();

    let mut possible_containers: HashMap<String, Vec<String>> = HashMap::new();
    for rule in re_rules.captures_iter(&buffer) {
        let parent = rule.name("parent").unwrap().as_str();
        let children = re_bags.captures_iter(rule.name("children").unwrap().as_str())
            .map(|child| child.name("color").unwrap().as_str().to_string())
            .collect::<Vec<String>>();

        for child in children.iter() {
            possible_containers.entry(child.to_string())
                .and_modify(|e| e.push(parent.to_string()))
                .or_insert(vec![parent.to_string()]);
        }
    }

    let possible_containers = possible_containers;
    let mut queue = VecDeque::new();
    let mut unique_containers = HashSet::new();
    queue.push_back("shiny gold".to_string());

    while let Some(bag) = queue.pop_front() {
        match possible_containers.get(&bag) {
            Some(containers) => {
                for bag in containers.iter() {
                    unique_containers.insert(bag.to_string());
                    queue.push_back(bag.to_string());
                }
            },
            None => { },
        };
    }

    println!("{:?}", unique_containers.len());
}
