extern crate regex;

use std::io::{self, Read};
use regex::Regex;
use std::collections::HashMap;

fn get_content_size(container: &str, possible_content: &HashMap<String, Vec<(i32, String)>>)
    -> i32 {
    match possible_content.get(container) {
        Some(bags) => {
            bags.iter()
                .map(|(n, bag)| n * (get_content_size(bag, possible_content) + 1))
                .sum()
        },
        None => { 0 },
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re_rules = Regex::new(r"(?P<parent>\w+ \w+) bags contain(?P<children>( \d+ \w+ \w+ bags?(?:,|\.))*)").unwrap();
    let re_bags = Regex::new(r"(?P<n>\d+) (?P<color>\w+ \w+) bags?").unwrap();

    let mut possible_content: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for rule in re_rules.captures_iter(&buffer) {
        let parent = rule.name("parent").unwrap().as_str().to_string();
        let children = re_bags.captures_iter(rule.name("children").unwrap().as_str())
            .map(|child| (
                    child.name("n").unwrap().as_str().parse().unwrap(),
                    child.name("color").unwrap().as_str().to_string(),
                    )
                )
            .collect::<Vec<(i32, String)>>();

        possible_content.insert(parent, children);
    }

    println!("{}", get_content_size("shiny gold", &possible_content));

}
