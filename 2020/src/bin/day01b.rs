fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let numbers = stdin.lock().lines()
        .map(|expense| expense.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let target = 2020;

    let (v1, v2, v3) = numbers
        .iter()
        .flat_map(|v1| numbers.iter().clone().map(move |v2| (v1, v2)))
        .flat_map(|(v1, v2)| numbers.iter().clone().map(move |v3| (v1, v2, v3)))
        .filter(|(&v1, &v2, &v3)| v1 + v2 + v3 == target).next().unwrap();


    println!("{}", v1*v2*v3);
}
