mod day01 {
    pub fn solve_a(depths: &[i32]) -> usize {
        depths.iter()
            .zip(depths.iter().skip(1))
            .filter(|(d1, d2)| d1 < d2)
            .count()
    }

    pub fn solve_b(depths: &[i32]) -> usize {
        let smooth_depths: Vec<i32> = depths.iter()
            .zip(depths.iter().skip(1).zip(depths.iter().skip(2)))
            .map(|(d1, (d2, d3))| d1 + d2 + d3)
            .collect();

        solve_a(&smooth_depths)
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let depths: Vec<i32> = stdin.lock().lines()
        .map(|d| d.unwrap().parse().unwrap())
        .collect();

    println!("Solution A-part: {}", day01::solve_a(&depths));
    println!("Solution B-part: {}", day01::solve_b(&depths));
}
