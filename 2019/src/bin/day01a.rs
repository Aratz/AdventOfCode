fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let res:f64 = stdin.lock().lines().map(|mass| (mass.unwrap().parse::<f64>().unwrap()/3.).floor() - 2.).sum();

    println!("{}", res);
}

