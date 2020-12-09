use std::io::{self, BufRead};

fn main() {
    let p = 25;

    let stdin = io::stdin();

    let numbers: Vec<i64> = stdin.lock().lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    let res = numbers[p..].iter().enumerate()
        .filter(|&(i, n)| {
            numbers[i..i+p].iter()
                .flat_map(|a| numbers[i..i+p].iter().clone().map(move |b| (a, b)))
                .filter(|(a, b)| a != b)
                .map(|(a, b)| a + b)
                .filter(|&s| s == *n)
                .count() == 0
        }).map(|(i, _)| numbers[i+p]).next().unwrap();

    println!("{:?}", res);
}
