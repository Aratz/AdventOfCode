use std::io::{self, BufRead};
use std::cmp::min;

fn phase(signal: &[i64]) -> Vec<i64> {
        let cum_s: Vec<i64> = signal.iter().scan(0, |acc, &x| {*acc += x; Some(*acc)}).collect();
        let mut x: Vec<i64> = Vec::with_capacity(signal.len()+10);

        let n = signal.len();

        x.push(signal[n-1]);

        for j in 2..=n {
            let i = n - j;
            let v = signal[i] + x[j-2] + (2..=n/(i+1)).map(|k| (-1 as i64).pow((k as u32)/2)*(cum_s[min(n-1, k*(i+2)-2)]-cum_s[k*(i+1)-2])).sum::<i64>();
            x.push(v);
        }

        x.reverse();
        x.iter().map(|x_i| x_i.abs()%10).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().next().unwrap().unwrap().chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    let n = input.len();
    let offset = input[..7].iter().map(|digit| digit.to_string()).collect::<Vec<String>>().join("").parse::<usize>().unwrap();
    println!("Offset: {}", offset);
    input = input.iter().cycle().take(n*10000).copied().collect();

    for i in 0..100 {
        println!("{}", i);

        let tmp = phase(&input);

        input = tmp;
    }

    let res = &input.iter().skip(offset).take(8).map(|digit| digit.to_string()).collect::<Vec<String>>().join("");
    println!("{}", res);
}
