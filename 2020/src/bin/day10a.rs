use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let mut ratings: Vec<i64> = stdin.lock().lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    ratings.insert(0, 0);
    ratings.push(ratings.iter().max().unwrap() + 3);

    ratings.sort();
    let diffs = ratings.iter().skip(1).zip(ratings.iter())
        .map(|(a1, a2)| a1 - a2).collect::<Vec<_>>();

    let mut counts = HashMap::new();

    for diff in diffs.iter() {
        counts.entry(diff)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    println!("{:?}", counts.values().product::<i64>());
}
