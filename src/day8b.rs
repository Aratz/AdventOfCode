use std::slice;

fn get_node_value(l:&mut slice::Iter<usize>) -> usize{
    let header = l.take(2).collect::<Vec<&usize>>();
    let n_node = *header[0];
    let n_metadata = *header[1];

    let mut child_nodes = Vec::new();
    for _ in 0..n_node {
        child_nodes.push(get_node_value(l));
    }

    if n_node == 0 {
        l.take(n_metadata).sum::<usize>()
    }
    else {
        l.take(n_metadata).filter(|&i| 0 < *i && *i <= n_node)
            .map(|i| child_nodes[i - 1]).sum::<usize>()
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let line = stdin.lock().lines().next().unwrap().unwrap().split(" ")
        .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    println!("{}", get_node_value(&mut line.iter()));
}
