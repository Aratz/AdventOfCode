use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let slope = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = vec![1, 3, 5, 7].iter().map(|&r| slope.iter().enumerate()
        .filter(|(i, row)| row[i*r % row.len()] == '#').count()).collect::<Vec<_>>();

    res.push(slope.iter().enumerate().step_by(2)
        .filter(|(i, row)| row[i/2 % row.len()] == '#').count());

    println!("{:?}", res.iter().product::<usize>());
}
