use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let slope = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let res = slope.iter().enumerate()
        .filter(|(i, row)| row[i*3 % row.len()] == '#').count();

    println!("{}", res);
}
