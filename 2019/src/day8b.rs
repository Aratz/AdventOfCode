fn main() {
    use std::io::{self, BufRead};

    const W: usize = 25;
    const H: usize = 6;

    let mut pic: [[char; W]; H] = [['.'; W]; H];

    let stdin = io::stdin();
    let pass = stdin.lock().lines().next().unwrap().unwrap()
        .chars().collect::<Vec<_>>();

    let pass_it = pass.chunks(W * H);

    for layer in pass_it {
        let layer = layer.chunks(W).enumerate();
        for (i, row) in layer {
            let row = row.iter().enumerate();
            for (j, c) in row {
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
