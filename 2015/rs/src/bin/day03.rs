mod day03 {
    use std::collections::HashSet;

    fn visit_houses(directions: &str) -> HashSet<(i32, i32)> {
        let mut houses = HashSet::new();
        let mut pos = (0, 0);
        houses.insert(pos);

        for dir in directions.chars() {
            pos = match dir {
                '>' => { (pos.0 + 1, pos.1) },
                'v' => { (pos.0, pos.1 - 1) },
                '<' => { (pos.0 - 1, pos.1) },
                '^' => { (pos.0, pos.1 + 1) },
                _ => unreachable!(),
            };

            houses.insert(pos);
        }

        houses
    }

    pub fn solve_a(input: &str) -> usize {
        visit_houses(input).len()
    }

    pub fn solve_b(input: &str) -> usize {
        let santa = visit_houses(&input.chars().step_by(2).collect::<String>());
        let robo_santa = visit_houses(&input.chars().skip(1).step_by(2).collect::<String>());

        santa.union(&robo_santa).count()
    }

    #[cfg(test)]
    mod test_day03 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(">"), 2);
            assert_eq!(solve_a("^>v<"), 4);
            assert_eq!(solve_a("^v^v^v^v^v"), 2);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b("^v"), 3);
            assert_eq!(solve_b("^>v<"), 3);
            assert_eq!(solve_b("^v^v^v^v^v"), 11);
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

    println!("Solution A-part: {}", day03::solve_a(&buffer));
    println!("Solution B-part: {}", day03::solve_b(&buffer));
}
