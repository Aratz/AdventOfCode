fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let numbers = stdin.lock().lines()
        .map(|expense| expense.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let target = 2020;

    let (v1, v2) = numbers
        .iter()
        .flat_map(|v1| numbers.iter().clone().map(move |v2| (v1, v2)))
        .find(|(&v1, &v2)| v1 + v2 == target).unwrap();


    println!("{}", v1*v2);
}
