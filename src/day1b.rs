fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut freq_changes = Vec::new();
    for frequency in stdin.lock().lines() {
        freq_changes.push(frequency.unwrap().parse::<i32>().unwrap());
    }

    let freq_sum = freq_changes.iter().sum::<i32>();

    let freq_changes_cp = freq_changes.clone();
    freq_changes.extend(&freq_changes_cp);

    let freq_sums = (0..freq_changes.len()+1)
        .map(|i| freq_changes[..i].iter().sum()).collect::<Vec<i32>>();

    let partial_sums = 
        (0..freq_changes.len()/2)
            .flat_map(|i| (i+1..i+freq_changes.len()/2+1)
                            .map(move |j| (i,j)));

    let min_index = partial_sums.map(|(i,j)| (freq_sums[i] - freq_sums[j], j, i))
        .filter(|&(diff, _i, _j)| 
                diff * freq_sum >= 0 // must have same signs
                && diff % freq_sum == 0)
        .min().unwrap();
    println!("{:?}",min_index);
    println!("{:?}",freq_changes[..min_index.2].iter().sum::<i32>());

}
