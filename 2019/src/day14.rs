extern crate regex;

use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

struct Reaction<'a> {
    reactants: Vec<(i64, &'a str)>,
    product: (i64, &'a str),
}

fn visit<'a>(n: &'a str, topology: &'a HashMap<&'a str, Reaction>,
         visited: &mut HashMap<&'a str, Option<bool>>, react_sorted: &mut Vec<&'a str>) {
    match visited[n] {
        Some(true) => return,
        Some(false) => {},
        None => panic!("Not a DAG!"),
    }

    visited.insert(n, None);

    for (_, m) in topology[n].reactants.iter() {
        visit(m, topology, visited, react_sorted);
    }

    visited.insert(n, Some(true));

    react_sorted.push(n);
}

fn main() {
    let re = Regex::new(r"(\d+) ([A-Z]+)").unwrap();

    let stdin = io::stdin();

    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let mut reactions: HashMap<&str, Reaction> = HashMap::new();

    for line in lines.iter() {
        let captures = re.captures_iter(line).collect::<Vec<_>>();
        let product = &captures[captures.len()-1];
        let product = (
            product.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            product.get(2).unwrap().as_str(),
            );

        let reactants = captures[..captures.len()-1].iter().map(|caps|
            (
                caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(2).unwrap().as_str(),
            )).collect::<Vec<(i64, &str)>>();

        reactions.insert(product.1, Reaction {reactants, product});
    }

    reactions.insert(&"ORE", Reaction { reactants:Vec::new(), product:(1, "ORE") });

    let reactions = reactions;

    // Topological sort

    let mut react_sorted: Vec<&str> = Vec::new();
    let mut visited: HashMap<&str, Option<bool>> = reactions.keys().map(|k| (*k, Some(false)))
        .collect();


    for n in reactions.keys() {
        visit(n, &reactions, &mut visited, &mut react_sorted);
    }

    react_sorted.remove(0);

    react_sorted.reverse();


    let react_sorted = react_sorted;

    let mut shopping_list: HashMap<&str, i64> = HashMap::new();
    shopping_list.insert("FUEL", 1);

    for product in react_sorted.iter() {
        let n = match shopping_list.remove(product) {
            Some(n) => n,
            None => continue,
        };
        let reaction = &reactions[product];
        let sp = reaction.product.0;

        let n_fired = n / sp + if n % sp > 0 { 1 } else { 0 };

        for (sr, reactant) in reaction.reactants.iter() {
            *shopping_list.entry(reactant).or_insert(0) += sr*n_fired;
        }
    }

    println!("{:?}", shopping_list);
}
