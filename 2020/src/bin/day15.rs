use std::io::{self, BufRead};

mod day15 {
    use std::collections::HashMap;

    pub fn solve(n: usize, numbers: &Vec<usize>) -> usize {
        let mut numbers = numbers.clone();
        let mut last_spoken: HashMap<usize, (Option<usize>, Option<usize>)> = HashMap::new();

        for i in 0..numbers.len() {
            last_spoken.insert(numbers[i], (Some(i+1), None));
        }

        for i in numbers.len()..n {
            let ith_spoken = match last_spoken.get(&numbers[i-1]).unwrap() {
                (Some(i1), Some(i2)) => { i1 - i2 },
                (Some(_), None) => { 0 },
                _ => { panic!("Malformed last spoken map!"); },
            };

            numbers.push(ith_spoken);
            last_spoken.entry(ith_spoken)
                .and_modify(|e| *e =  (Some(i+1), e.0))
                .or_insert((Some(i+1), None));
        }

        *numbers.last().unwrap()
    }
}

fn main() {
    let stdin = io::stdin();

    let numbers: Vec<usize> = stdin.lock().lines().next().unwrap().unwrap()
        .split(",").map(|n| n.parse().unwrap()).collect();


    println!("Solution A-part: {}", day15::solve(2_020, &numbers));
    println!("Solution B-part: {}", day15::solve(30_000_000, &numbers));
}
