fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let (_, finish) = stdin.lock().lines().next().unwrap().unwrap()
        .split(", ").map(|s| {
            let mut ch = s.chars();
            let turn = match ch.next().unwrap() {
                'R' => 1,
                'L' => -1,
                _ => panic!("Input error!"),
            };
            let dist = ch.collect::<String>().parse::<i32>().unwrap();
            (turn, dist)
        })
        .fold((0, (0, 0)), |(dir, (x, y)), (turn, dist)| {
            let dir = (dir + turn + 4)%4;
            let (dx, dy) = match dir {
                0 => (0, 1),
                1 => (1, 0),
                2 => (0, -1),
                3 => (-1, 0),
                _ => panic!("Direction error!"),
            };
            (dir, (x + dist*dx, y + dist*dy))
        });

    println!("{}", finish.0.abs() + finish.1.abs());
}
