fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let skill_imp = stdin.lock().lines().next().unwrap().unwrap();

    let mut scoreboard = vec![3, 7];

    let (mut rec_1, mut rec_2) = (0, 1);

    for i in (0..10000) {
        println!("{}", i);
        let new_rec = scoreboard[rec_1] + scoreboard[rec_2];
        if new_rec >= 10 {
            scoreboard.push(new_rec / 10);
        }
        scoreboard.push(new_rec % 10);

        rec_1 = (rec_1 + scoreboard[rec_1] + 1) % scoreboard.len();
        rec_2 = (rec_2 + scoreboard[rec_2] + 1) % scoreboard.len();
    }

    match scoreboard.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
        .find(skill_imp) {
        Some(index) => println!("{}", index),
        None => println!("404 recipes not found"),
    }
}
