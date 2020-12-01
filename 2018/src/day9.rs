use std::collections::HashMap;

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let pn = stdin.lock().lines().next().unwrap().unwrap().split(" ")
        .map(|s| s.parse::<usize>()).filter_map(Result::ok)
        .collect::<Vec<usize>>();

    let (p, n) = (pn[0], pn[1]);

    let mut marbles = vec![0];

    let mut pos = 0;
    let mut scores = HashMap::new();
    let mut player = 0;

    for marble in 1..=n {
        if marble % 23 != 0 {
            let i_pos =  (pos + 2) % marbles.len();
            marbles.insert(i_pos, marble);
            pos = (pos + 2) % (marbles.len() - 1);
        }
        else {
            let score = scores.entry(player).or_insert(0);
            *score += marble + marbles[(pos + marbles.len() - 7) % marbles.len()];

            let r_pos = (pos + marbles.len() - 7) % marbles.len();
            marbles.remove(r_pos);

            pos = r_pos % marbles.len();

        }

        player = (player + 1) % p;

        if marble % 10000 == 0 {
            println!("{}", marble);
        }
    }
    println!("{}", scores.values().max().unwrap());
}
