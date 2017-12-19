fn main(){
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut result = 0;
    for line in stdin.lock().lines() {
        let numbers:Vec<i32> = line.unwrap().split("\t").map(|s| s.parse::<i32>().unwrap()).collect();
        let max_n = numbers.iter().max().unwrap();
        let min_n = numbers.iter().min().unwrap();

        result += max_n - min_n;
    }

    println!("{}", result);
}
