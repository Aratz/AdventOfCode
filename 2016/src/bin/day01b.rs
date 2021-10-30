use std::collections::HashSet;

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let turns = stdin.lock().lines().next().unwrap().unwrap()
        .split(", ").map(|s| {
            let mut ch = s.chars();
            let turn = match ch.next().unwrap() {
                'R' => 1,
                'L' => -1,
                _ => panic!("Input error!"),
            };
            let dist = ch.collect::<String>().parse::<i32>().unwrap();
            (turn, dist)
        }).collect::<Vec<_>>();

    let mut dir = 0;
    let (mut x, mut y) = (0, 0);
    let mut prev: HashSet<(i32, i32)> = HashSet::new();

    prev.insert((x, y));

    for (turn, dist) in turns.iter() {
        dir = (dir + turn + 4)%4;
        let (dx, dy) = match dir {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => panic!("Direction error!"),
        };

        for _ in 0..*dist {
            x += dx;
            y += dy;
            if prev.contains(&(x, y)) {
                println!("{}", x.abs() + y.abs());
                return
            }
            else {
                prev.insert((x, y));
            }
        }
    }
}
