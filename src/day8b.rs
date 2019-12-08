fn main() {
    use std::io::{self, BufRead};

    const W: usize = 25;
    const H: usize = 6;

    let mut pic: [[char; W]; H] = [['.'; W]; H];

    let stdin = io::stdin();
    let pass = stdin.lock().lines().next().unwrap().unwrap()
        .chars().collect::<Vec<_>>();

    let mut pass_it = pass.chunks(W * H);

    while let Some(layer) = pass_it.next() {
        let mut layer = layer.chunks(W).enumerate();
        while let Some((i, row)) = layer.next() {
            let mut row = row.iter().enumerate();
            while let Some((j, c)) = row.next() {
                pic[i][j] = match pic[i][j] {
                    '.' => match c {
                        '0' => ' ',
                        '1' => '█',
                        _ => '.',
                    },
                    _ => pic[i][j],
                };
            }
        }
    }

    let rows = (0..H).map(|i| pic[i].iter().collect::<String>()).collect::<Vec<_>>();
    println!("{}", rows.join("\n"));
}
