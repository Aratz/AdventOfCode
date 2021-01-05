extern crate regex;

mod day20 {
    #[derive(Clone, Copy, Debug)]
    pub struct Particle {
        pub p: [i64; 3],
        pub v: [i64; 3],
        pub a: [i64; 3],
    }

    #[inline]
    fn norm(a: &[i64; 3]) -> i64 {
        (0..3).map(|i| a[i].abs()).sum()
    }

    pub fn solve_a(particles: &[Particle]) -> usize {
        let mn = particles.iter().enumerate()
            .map(|(i, p)| (norm(&p.a), norm(&p.v), norm(&p.p), i))
            .min().unwrap();

        mn.3
    }

    #[cfg(test)]
    mod test_day20 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let particles = vec![
                Particle { p: [3, 0, 0], v: [2, 0, 0], a: [-1, 0, 0] },
                Particle { p: [4, 0, 0], v: [0, 0, 0], a: [-2, 0, 0] },
            ];

            assert_eq!(solve_a(&particles), 0);
        }
    }
}

fn main() {
    use regex::Regex;
    use std::io::{self, Read};
    use std::convert::TryInto;

    let stdin = io::stdin();
    let mut buffer = String::new();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re_particle = Regex::new(r"p=<(?P<p>(-?\d+,?){3})>, v=<(?P<v>(-?\d+,?){3})>, a=<(?P<a>(-?\d+,?){3})>").unwrap();

    let particles: Vec<_> = re_particle.captures_iter(&buffer)
        .map(|capt| {
            let p = capt.name("p").unwrap().as_str()
                .split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>()
                .try_into().unwrap();
            let v = capt.name("v").unwrap().as_str()
                .split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>()
                .try_into().unwrap();
            let a = capt.name("a").unwrap().as_str()
                .split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>()
                .try_into().unwrap();

            day20::Particle { p, v, a }
        }).collect();

    println!("Solution A-part: {}", day20::solve_a(&particles));
}
