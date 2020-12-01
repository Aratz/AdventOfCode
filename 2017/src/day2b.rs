fn main(){
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut result = 0;
    for line in stdin.lock().lines() {
        let mut numbers:Vec<i32> = line.unwrap().split("\t").map(|s| s.parse::<i32>().unwrap()).collect();
        numbers.sort();

        for (i, denominator) in numbers.iter().enumerate() {
            for numerator in numbers.iter().skip(i + 1) {
                if numerator % denominator == 0 {
                    result += numerator / denominator;
                }
            }
        }
    }

    println!("{}", result);
}
