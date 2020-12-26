mod day13 {
    use std::collections::HashMap;


    pub fn solve_a(layers: &HashMap<usize, usize>) -> usize {
        layers.iter().filter(|(depth, range)| *depth % (2 * (*range - 1)) == 0)
            .map(|(depth, range)| depth * range)
            .sum()
    }

    pub fn solve_b(layers: &HashMap<usize, usize>) -> usize {
        (0..).find(|&delay| {
                layers.iter()
                    .filter(|(depth, range)| (*depth + delay) % (2 * (*range - 1)) == 0)
                    .count() == 0
            }).unwrap()
    }

    #[cfg(test)]
    mod test_day13 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let layers = vec![(0, 3), (1,2), (4, 4), (6, 4)].into_iter().collect();

            assert_eq!(solve_a(&layers), 24);
        }

        #[test]
        fn test_solve_b() {
            let layers = vec![(0, 3), (1,2), (4, 4), (6, 4)].into_iter().collect();

            assert_eq!(solve_b(&layers), 10);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let firewall = stdin.lock().lines().map(
        |l| {
            let dr: Vec<_> = l.unwrap().split(": ").map(|v| v.parse().unwrap())
                .collect();
            (dr[0], dr[1])
        }).collect();

    println!("Solution A-part: {}", day13::solve_a(&firewall));
    println!("Solution B-part: {}", day13::solve_b(&firewall));
}
