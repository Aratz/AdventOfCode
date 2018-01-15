use std::io::{self, BufRead};

fn main(){
    let n = 256;
    let n_rounds = 64;
    let mut vec = (0..256).collect::<Vec<usize>>();
    let suffix = vec![17, 31, 73, 47, 23];

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut skip_size = 0;
    let mut shift = 0;

    for _ in 0..n_rounds{
        let mut lengths = line.chars().map(|x| (x as u8) as usize).collect::<Vec<usize>>();
        lengths.extend(suffix.iter());
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
    }

    vec = vec.iter().cycle().skip(n - (shift % n)).take(n).map(|x| *x).collect::<Vec<usize>>();

    for i in 0..16{
        print!("{:2.x}", vec[16*i..16*(i+1)].iter().fold(0, |acc, &x| acc ^ x));
    }

    println!("");
}
