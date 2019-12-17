use std::io::{self, BufRead};
use std::iter;
use std::cmp::min;

fn phase(signal: &Vec<i64>, pattern: &Vec<i64>) -> Vec<i64> {
        let cum_s: Vec<i64> = signal.iter().scan(0, |acc, &x| {*acc = *acc + x; Some(*acc)}).collect();
        let mut X: Vec<i64> = Vec::with_capacity(signal.len()+10);

        let N = signal.len();

        X.push(signal[N-1]);

        for j in 2..N+1 {
            let i = N - j;
            let v = (signal[i] + X[j-2] + (2..N/(i+1)+1).map(|k| (-1 as i64).pow((k as u32)/2)*(cum_s[min(N-1, k*(i+2)-2)]-cum_s[k*(i+1)-2])).sum::<i64>());
            X.push(v);
        }

        X.reverse();
        X.iter().map(|x| x.abs()%10).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().next().unwrap().unwrap().chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    let N = input.len();
    let offset = input[..7].iter().map(|n| n.to_string()).collect::<Vec<String>>().join("").parse::<usize>().unwrap();
    println!("Offset: {}", offset);
    input = input.iter().cycle().take(N*10000).map(|&x| x).collect();

    let pattern = vec![0, 1, 0, -1];

    for i in 0..100 {
        println!("{}", i);

        let mut tmp = phase(&input, &pattern);

        input = tmp;
    }

    let res = &input.iter().skip(offset).take(8).map(|n| n.to_string()).collect::<Vec<String>>().join("");
    println!("{}", res);
}
