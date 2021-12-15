extern crate regex;
extern crate num_integer;

use std::io::{self, BufRead};
use std::collections::HashMap;
use num_integer:: Integer;
use std::fmt;
use regex::Regex;

struct Reaction<'a> {
    reactants: Vec<(i64, &'a str)>,
    product: (i64, &'a str),
}

impl<'a> fmt::Display for Reaction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}",
               self.reactants.iter().map(|(s, r)| format!("{} {}", s, r))
               .collect::<Vec<String>>().join(" + "),
               format!("{} {}", self.product.0, self.product.1))
    }
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


    let mut ore_react: HashMap<&str, Reaction> = HashMap::new();

    for product in react_sorted.iter() {
        let reactants = &reactions[product].reactants;

        if reactants[0].1 == "ORE" {
            ore_react.insert(product, Reaction {
                reactants: vec![reactants[0]],
                product:(reactions[product].product.0, product),
            });
        }
        else {
            let bcoef: i64 = reactants.iter()
                .map(|(s, r)| s.lcm(&ore_react[r].product.0)/s)
                .fold(1, |acc, x| acc.lcm(&x));

            let n_ore: i64 = reactants.iter()
                .map(|(s, r)| s*bcoef/ore_react[r].product.0*ore_react[r].reactants[0].0)
                .sum();

            let n_product = reactions[product].product.0*bcoef;

            ore_react.insert(&product, Reaction {
                reactants:vec![(n_ore/(n_ore.gcd(&n_product)), "ORE")],
                product:(n_product/(n_ore.gcd(&n_product)), product),
            });
        }
    }
    println!("{}", ore_react["FUEL"]);

    let fuel_per_ore = (
        ore_react["FUEL"].product.0,
        ore_react["FUEL"].reactants[0].0,
        );

    let cargo_ore: i64 = 1_000_000_000_000;
    println!("Residual ore: {}, fuel: {}", fuel_per_ore.1, fuel_per_ore.0);
    println!("Total Fuel: {}",
             ((cargo_ore as f64) * (fuel_per_ore.0 as f64) / (fuel_per_ore.1 as f64)).floor());

}
