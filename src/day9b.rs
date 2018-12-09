use std::collections::HashMap;


fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let pn = stdin.lock().lines().next().unwrap().unwrap().split(" ")
        .map(|s| s.parse::<usize>()).filter_map(Result::ok)
        .collect::<Vec<usize>>();

    let (p, n) = (pn[0], pn[1]);


    let mut scores = HashMap::new();
    let mut marbles = HashMap::new();
    let mut player = 0;
    marbles.insert(0, (0, 0));

    let mut current_marble:usize = 0;

    for marble in 1..=n {
        if marble % 23 != 0 {
            current_marble = marbles.get(&current_marble).unwrap().1;

            let next_marble = marbles.get(&current_marble).unwrap().1;
            marbles.insert(marble, (current_marble, next_marble));
            marbles.entry(current_marble).and_modify(|e| (*e).1 = marble);
            marbles.entry(next_marble).and_modify(|e| (*e).0 = marble);

            current_marble = marbles.get(&current_marble).unwrap().1;
        }
        else {
            for _ in 0..6 {
                current_marble = marbles.get(&current_marble).unwrap().0;
            }

            let extra_marble = marbles.get(&current_marble).unwrap().0;
            let pre_extra_marble = marbles.get(&extra_marble).unwrap().0;

            let score = scores.entry(player).or_insert(0);
            *score += marble + extra_marble;

            marbles.entry(pre_extra_marble).and_modify(|e| (*e).1 = current_marble);
            marbles.remove(&extra_marble);
            marbles.entry(current_marble).and_modify(|e| (*e).0 = pre_extra_marble);
        }

        player = (player + 1) % p;
    }
    println!("{}", scores.values().max().unwrap());
}
