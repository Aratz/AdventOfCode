extern crate regex;

use regex::Regex;
use std::io::{self, Read};
use std::collections::hash_map::HashMap;


#[derive(Debug)]
struct Program<'a> {
    name: &'a str,
    own_weight: u32,
    children: Vec<&'a str>,
    total_weight: Option<u32>,
    parent: Option<&'a str>,
    unbalanced: bool,
}

fn get_total_weight<'a>(parent_name: &'a str, mut programs: &mut HashMap<&'a str, Program<'a>>) -> u32 {
    let current_weight = programs[parent_name].total_weight;

    match current_weight {
        Some(total_weight) => total_weight,
        None => {
            let children = programs[parent_name].children.to_vec();
            let weights = children.iter()
                .map(|child| get_total_weight(child, &mut programs)).collect::<Vec<_>>();

            programs.entry(parent_name)
                .and_modify(|p| p.unbalanced = match weights.get(0) {
                    Some(first_weight) => weights.iter()
                        .filter(|&w| w == first_weight).count() != weights.len(),
                    None => false,
                });

            let partial_weight: u32 = weights.iter().sum();
            let own_weight = programs[parent_name].own_weight;

            programs.entry(parent_name)
                .and_modify(|p| p.total_weight = Some(partial_weight + own_weight));

            partial_weight + own_weight
        },
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re = Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<children>(?:\w+,\s)*\w+))?").unwrap();

    let mut programs: HashMap<&str, Program> = re.captures_iter(&buffer).map(|c| (
            c.name("name").unwrap().as_str(),
            Program {
                name: c.name("name").unwrap().as_str(),
                own_weight: c.name("weight").unwrap().as_str().parse().unwrap(),
                children: match c.name("children") {
                    Some(raw_children) => raw_children.as_str().split(", ").collect(),
                    None => Vec::new(),
                },
                total_weight: match c.name("children") {
                    Some(_) => None,
                    None => Some(c.name("weight").unwrap().as_str().parse().unwrap()),
                },
                parent: None,
                unbalanced: false,
            }
        )
    ).collect();

    let parent_names = programs.keys().map(|s| *s).collect::<Vec<&str>>();
    for parent_name in parent_names.iter() {
        let children = programs[parent_name].children.to_vec();
        for child in children.iter() {
            programs.entry(child).and_modify(|p| p.parent = Some(parent_name));
        }
    }

    for parent_name in parent_names.iter() {
        get_total_weight(parent_name, &mut programs);
    }

    let unbalanced_program = programs.values()
        .filter(|p| p.unbalanced
                && p.children.iter()
                    .filter(|&c| programs[c].unbalanced)
                    .count() == 0)
        .next().unwrap();

    let mut weight_child = unbalanced_program.children.iter()
        .map(|child| (programs[child].total_weight.unwrap(), child)).collect::<Vec<_>>();

    weight_child.sort();

    let median_weight = weight_child[weight_child.len()/2].0;

    let fat_child = weight_child.iter().filter(|(w, _)| *w != median_weight).next().unwrap();
    //adjust it's weight to be equal to all the others

    println!("{:?}", programs[fat_child.1].own_weight + median_weight - fat_child.0);
}
