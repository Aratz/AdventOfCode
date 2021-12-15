use std::io::{self, BufRead};

fn main() {
    let p = 25;

    let stdin = io::stdin();

    let numbers: Vec<i64> = stdin.lock().lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    let n_inv = numbers[p..].iter().enumerate()
        .filter(|&(i, n)| {
            numbers[i..i+p].iter()
                .flat_map(|a| numbers[i..i+p].iter().clone().map(move |b| (a, b)))
                .filter(|(a, b)| a != b)
                .map(|(a, b)| a + b)
                .filter(|&s| s == *n)
                .count() == 0
        }).map(|(i, _)| numbers[i+p]).next().unwrap();

    let mut cum_sum: Vec<i64> = numbers.iter().scan(0, |state, &x| {
            *state += x;
            Some(*state)
        }).collect();
    cum_sum.insert(0, 0);
    //remember indices are shifted by 1

    let (i, j) = (0..cum_sum.len()).flat_map(|i| (0..cum_sum.len()).map(move |j| (i, j)))
        .filter(|&(i, j)| j > i)
        .find(|&(i, j)| cum_sum[j] - cum_sum[i] == n_inv).unwrap();

    let res = numbers[i..j].iter().min().unwrap() + numbers[i..j].iter().max().unwrap();

    println!("{}", res);
}
