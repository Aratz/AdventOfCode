extern crate rulinalg;

fn main(){
    use std::io::{self, BufRead};
    use std::collections::BTreeSet;

    let stdin = io::stdin();

    let mut banks:Vec<i32> = stdin.lock().lines().next().unwrap().unwrap().split("\t")
        .map(|s| s.parse::<i32>().unwrap()).collect();

    let mut patterns = BTreeSet::new();

    let mut counter = 0;

    while !patterns.contains(&banks) {
        patterns.insert(banks.clone());

        use rulinalg::utils;

        let mut i = utils::argmax(&banks).0;
        let mut nblocks = banks[i];
        banks[i] = 0;
        while nblocks > 0 {
            i = (i + 1) % banks.len();
            banks[i] += 1;
            nblocks -= 1;
        }

        counter += 1;
    }

    counter = 0;
    let pattern = banks.clone();

    {
        use rulinalg::utils;

        let mut i = utils::argmax(&banks).0;
        let mut nblocks = banks[i];
        banks[i] = 0;
        while nblocks > 0 {
            i = (i + 1) % banks.len();
            banks[i] += 1;
            nblocks -= 1;
        }
    }

    while pattern != banks {
        use rulinalg::utils;

        let mut i = utils::argmax(&banks).0;
        let mut nblocks = banks[i];
        banks[i] = 0;
        while nblocks > 0 {
            i = (i + 1) % banks.len();
            banks[i] += 1;
            nblocks -= 1;
        }

        counter += 1;
    }

    println!("{}", counter + 1);
}
