use std::slice;

fn read_nodes(l:&mut slice::Iter<usize>) -> usize{
    let header = l.take(2).collect::<Vec<&usize>>();
    let n_node = *header[0];
    let n_metadata = *header[1];

    let mut checksum = 0;
    for _ in 0..n_node {
        checksum += read_nodes(l);
    }

    checksum + l.take(n_metadata).sum::<usize>()
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let line = stdin.lock().lines().next().unwrap().unwrap().split(" ")
        .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    println!("{}", read_nodes(&mut line.iter()));
}
