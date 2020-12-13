fn get_fuel(mut mass:f64) -> f64 {
    let mut res = 0.;
    loop {
        mass = (mass/3.).floor() - 2.;

        if mass <= 0. {break}

        res += mass;
    }
    res
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let res:f64 = stdin.lock().lines().map(|mass| get_fuel(mass.unwrap().parse::<f64>().unwrap())).sum();

    println!("{}", res);
}

