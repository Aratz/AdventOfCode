mod day24 {
    const BLOCK_SIZE: usize = 18;
    const N_BLOCKS: usize = 14;

    fn split_input(input: &[String]) -> Vec<Vec<String>> {
        (0..N_BLOCKS)
            .map(|i| input[BLOCK_SIZE*i..BLOCK_SIZE*(i+1)].to_vec())
            .collect()
    }

    fn extract_abc(block: &[String]) -> (i64, i64, i64) {
        (
            block[5].split(' ').last().unwrap().parse().unwrap(),
            block[15].split(' ').last().unwrap().parse().unwrap(),
            block[4].split(' ').last().unwrap().parse().unwrap(),
        )
    }

    pub fn solve(prgm: &[String], ab: bool) -> i64 {
        let mut input = [0; 14];

        let mut stack = Vec::new();
        for (i, (a, b, c)) in split_input(prgm)
            .into_iter()
            .map(|block| extract_abc(&block))
            .enumerate() {
            if c == 1 {
                stack.push((i, b));
            }
            else {
                let (i_w, b) = stack.pop().unwrap();
                let w01 = (1..=9)
                    .map(|w| (w, w + b + a))
                    .filter(|&(_, w)| (1..=9).contains(&w))
                    .collect::<Vec<_>>();

                input[i_w] = w01[if ab { 0 } else { w01.len() - 1 }].0;
                input[i] = w01[if ab { 0 } else { w01.len() - 1 }].1;
            }
        }

        input.iter().rev().enumerate()
            .map(|(i, w)| w*10i64.pow(i as u32))
            .sum::<i64>()
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let input: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    println!("Solution A-part: {}", day24::solve(&input, false));
    println!("Solution B-part: {}", day24::solve(&input, true));
}
