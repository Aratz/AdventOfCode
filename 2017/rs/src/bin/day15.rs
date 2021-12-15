mod day15 {
    const BASE: u64 = 2_147_483_647;

    struct Generator {
        seed: u64,
        factor: u64,
        criteria: u64,
    }

    impl Generator {
        fn new(seed: u64, factor: u64, criteria: u64) -> Self {
            Generator { seed, factor, criteria }
        }
    }

    impl Iterator for Generator {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            self.seed = (self.seed * self.factor) % BASE;

            while self.seed % self.criteria != 0 {
                self.seed = (self.seed * self.factor) % BASE;
            }

            Some(self.seed)
        }
    }

    pub fn solve_a(seed_a: u64, seed_b: u64) -> usize {
        let gen_a = Generator::new(seed_a, 16_807, 1);
        let gen_b = Generator::new(seed_b, 48_271, 1);

        gen_a.zip(gen_b).take(40_000_000)
            .filter(|(a, b)| a % (1<<16) == b % (1<<16))
            .count()
    }

    pub fn solve_b(seed_a: u64, seed_b: u64) -> usize {
        let gen_a = Generator::new(seed_a, 16_807, 4);
        let gen_b = Generator::new(seed_b, 48_271, 8);

        gen_a.zip(gen_b).take(5_000_000)
            .filter(|(a, b)| a % (1<<16) == b % (1<<16))
            .count()
    }

    #[cfg(test)]
    mod test_day15 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(65, 8_921), 588);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(65, 8_921), 309);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let seed_a = lines.next().unwrap().unwrap()
        .split(' ').last().unwrap()
        .parse().unwrap();
    let seed_b = lines.next().unwrap().unwrap()
        .split(' ').last().unwrap()
        .parse().unwrap();

    println!("Solution A-part: {}", day15::solve_a(seed_a, seed_b));
    println!("Solution B-part: {}", day15::solve_b(seed_a, seed_b));
}
