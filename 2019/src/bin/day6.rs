extern crate itertools;

use std::collections::HashMap;
use std::io::{self, Read};
use itertools::Itertools;

fn distance_to_com(data:&HashMap<&str, &str>, p:&str) -> i32 {
    if p == "COM" {
        0
    }
    else {
        1 + distance_to_com(data, data[p])
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = String::new();

    stdin.lock().read_to_string(&mut lines).unwrap();

    let direct_orbits: HashMap<&str, &str> = lines.lines()
        .flat_map(|l| l.split(')'))
        .tuples().map(|(a, b)| (b, a)).collect();

    println!("{}",
        direct_orbits.iter().map(|(p, _)| distance_to_com(&direct_orbits, p)).sum::<i32>());

}
