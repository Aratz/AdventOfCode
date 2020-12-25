use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut ratings: Vec<i64> = stdin.lock().lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    ratings.insert(0, 0);
    ratings.push(ratings.iter().max().unwrap() + 3);

    ratings.sort_unstable();

    let mut n_arr: Vec<i64> = vec![0; ratings.len()];

    n_arr[0] = 1;

    for i in 1..ratings.len() {
        n_arr[i] = 
            (if i >= 1 && ratings[i] - ratings[i-1] <= 3 { n_arr[i-1] } else { 0 })
            + (if i >= 2 && ratings[i] - ratings[i-2] <= 3 { n_arr[i-2] } else { 0 })
            + (if i >= 3 && ratings[i] - ratings[i-3] <= 3 { n_arr[i-3] } else { 0 });
    }

    println!("{}", n_arr.last().unwrap());

}
