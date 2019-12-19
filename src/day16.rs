use std::io::{self, BufRead};
use std::iter;

fn phase(signal: &[i64], pattern: &[i64]) -> Vec<i64> {
        signal.iter().enumerate()
        .map(|(i, _val)| pattern.iter().flat_map(|c| iter::repeat(c).take(i + 1))
             .cycle().skip(1)
             .zip(signal).map(|(p, s)| p*s).sum::<i64>().abs() % 10).collect::<Vec<i64>>()
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().next().unwrap().unwrap().chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    let pattern = vec![0, 1, 0, -1];

    for _ in 0..100 {
        let tmp = phase(&input, &pattern);

        input = tmp;
    }
    let res = &input.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("")[..8];
    println!("{}", res);
}
