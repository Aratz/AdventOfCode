extern crate regex;

mod day14 {
    use regex::Regex;
    use std::cmp::min;

    struct Reindeer {
        speed: usize,
        fly_time: usize,
        rest_time: usize,
    }

    impl Reindeer {
        fn dist(&self, time: usize) -> usize {
            self.speed * self.fly_time * (time / (self.fly_time + self.rest_time))
                + min(time % (self.fly_time + self.rest_time), self.fly_time)
                    * self.speed
        }
    }

    fn parse(input: &str) -> Vec<Reindeer> {
        let re_reindeer = Regex::new(r"\w+ can fly (?P<speed>\d+) km/s for (?P<fly_time>\d+) seconds, but then must rest for (?P<rest_time>\d+) seconds.").unwrap();

        re_reindeer.captures_iter(input)
            .map(|capt| Reindeer {
                speed: capt["speed"].parse().unwrap(),
                fly_time: capt["fly_time"].parse().unwrap(),
                rest_time: capt["rest_time"].parse().unwrap(),
            }).collect()
    }

    pub fn solve_a(input: &str, time: usize) -> usize {
        let reindeers = parse(input);

        reindeers.iter()
            .map(|rndr| rndr.dist(time))
            .max().unwrap()
    }

    pub fn solve_b(input: &str, time: usize) -> usize {
        let reindeers = parse(input);

        let dists = reindeers.iter()
            .map(|rndr| (1..=time).map(|t| rndr.dist(t)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let lead_dist = (0..time)
            .map(|t| dists.iter().map(|v| v[t]).max().unwrap())
            .collect::<Vec<_>>();

        dists.iter()
            .map(|v| v.iter()
                 .zip(lead_dist.iter())
                 .filter(|(t_rndr, t_lead)| t_rndr == t_lead)
                 .count())
            .max().unwrap()
    }

    #[cfg(test)]
    mod test_day14 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

            assert_eq!(solve_a(&input, 1), 16);
            assert_eq!(solve_a(&input, 1000), 1120);
        }

        #[test]
        fn test_solve_b() {
            let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

            assert_eq!(solve_b(&input, 1000), 689);
        }
    }

}

fn main() {
    use std::io::{self, Read};

    let stdin = io::stdin();

    let mut buffer = String::new();

    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    println!("Solution A-part: {}", day14::solve_a(&buffer.trim(), 2503));
    println!("Solution B-part: {}", day14::solve_b(&buffer.trim(), 2503));
}
