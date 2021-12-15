mod day15 {
    #[derive(Clone)]
    pub struct Disc {
        id: i32,
        n_pos: i32,
        init_pos: i32,
    }

    impl Disc {
        pub fn new(s: &str) -> Self {
            let words: Vec<&str> = s.split(" ").collect();

            let n_pos = words[3].parse().unwrap();
            let init_pos = words[11][..words[11].len()-1].parse().unwrap();
            let id = words[1][1..].parse::<i32>().unwrap()%n_pos;

            Self { id, n_pos, init_pos }
        }
    }

    /// Compute s and t such that a*s + b*t = gcd(a, b)
    ///
    /// Returns (gcd(a, b), (s, t))
    ///
    /// This is an implementation of the [extended Euclidian algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
    #[allow(clippy::many_single_char_names)]
    fn xgcd(a: i32, b: i32) -> (i32, (i32, i32)) {
        let (mut old_r, mut r) = (a, b);
        let (mut old_s, mut s) = (1, 0);
        let (mut old_t, mut t) = (0, 1);

        while r != 0 {
            let quotient = old_r / r;

            let temp_r = old_r;
            old_r = r;
            r = temp_r - quotient*r;

            let temp_s = old_s;
            old_s = s;
            s = temp_s - quotient*s;

            let temp_t = old_t;
            old_t = t;
            t = temp_t - quotient*t;

        }

        (old_r, (old_s, old_t))
    }

    fn first_time(discs: &[Disc]) -> i32 {
        let a: Vec<i32> = discs.iter()
            .map(|disc| disc.n_pos - (disc.id + disc.init_pos)%disc.n_pos)
            .collect();

        let n: Vec<i32> = discs.iter()
            .map(|disc| disc.n_pos)
            .collect();

        let n_prod: i32 = n.iter().product();

        let n_hat: Vec<i32> = n.iter()
            .map(|ni| n_prod / ni)
            .collect();

        let v: Vec<i32> = n.iter().zip(n_hat.iter())
            .map(|(&ni, &ni_hat)| (xgcd(ni, ni_hat).1).1)
            .collect();

        let e: Vec<i32> = v.iter().zip(n_hat.iter())
            .map(|(vi, ni_hat)| vi*ni_hat)
            .collect();

        let mut res: i32 = e.iter().zip(a.iter())
            .map(|(ei, ai)| ei*ai)
            .sum();

        while res < 0 {
            res += n_prod;
        }

        res % n_prod
    }

    pub fn solve_a(discs: &[Disc]) -> i32 {
        first_time(discs)
    }

    pub fn solve_b(discs: &[Disc]) -> i32 {
        let mut discs = discs.to_vec();

        discs.push(
            Disc {
                id: discs.len() as i32 + 1,
                n_pos: 11,
                init_pos: 0,
            });

        first_time(&discs)
    }

}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let discs: Vec<day15::Disc> = stdin.lock().lines()
        .map(|l| day15::Disc::new(&l.unwrap()))
        .collect();

    println!("Solution A-part: {}", day15::solve_a(&discs));
    println!("Solution B-part: {}", day15::solve_b(&discs));
}
