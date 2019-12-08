fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let pass = stdin.lock().lines().next().unwrap().unwrap();

    const W: usize = 25;
    const H: usize = 6;

    let mut pic: [[char; W]; H] = [['.'; W]; H];

    let mut pass_it = pass.chars().enumerate();

    let mut i = 0;
    let mut j = 0;

    while let Some((k, c)) = pass_it.next() {
        if k % (W * H) == 0 && k != 0 {
            i = 0;
            j = 0;
        }
        else if k % W == 0 && k != 0 {
            i += 1;
            j = 0;
        }

        pic[i][j] = match pic[i][j] {
            '.' => match c {
                '0' => ' ',
                '1' => '█',
                _ => '.',
            },
            _ => pic[i][j],
        };

        j += 1;
    }

    for j in 0..H {
        println!("{:?}", pic[j].iter().collect::<String>());
    }
}
