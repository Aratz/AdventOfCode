use std::io::{self, BufRead};

fn main(){
    let n = 256;
    let mut vec = (0..n).collect::<Vec<usize>>();

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let lengths = line.split(",").map(|x| x.parse::<usize>().unwrap());
    let mut skip_size = 0;
    let mut shift = 0;

    for length in lengths {
        vec = vec.iter().cycle().take(length).map(|x| *x).collect::<Vec<usize>>()
            .iter().rev().map(|x| *x).collect::<Vec<usize>>()
            .iter().map(|x| *x).chain(
                vec.iter().cycle().skip(length).take(n-length).map(|x| *x))
            .collect::<Vec<usize>>();

        vec = vec.iter().cycle().skip(length + skip_size).take(n).map(|x| *x).collect::<Vec<usize>>();
        shift += length + skip_size;

        skip_size += 1;
    }

    vec = vec.iter().cycle().skip(n - (shift % n)).take(n).map(|x| *x).collect::<Vec<usize>>();
    println!("{}", vec[0] * vec[1]);
}
