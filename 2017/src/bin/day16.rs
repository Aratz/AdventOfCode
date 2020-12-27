mod day16 {
    pub enum DanceMove {
        Spin(usize),
        Exchange(usize, usize),
        Partner(char, char),
    }

    impl DanceMove {
        fn execute(&self, programs: &mut Vec<char>) {
            match *self {
                Self::Spin(x) => {
                    let n = programs.len();
                    *programs = programs.iter()
                        .cycle()
                        .skip(n - x)
                        .take(n)
                        .copied()
                        .collect();
                },
                Self::Exchange(a, b) => {
                    programs.swap(a, b)
                },
                Self::Partner(a, b) => {
                    let i_a = programs.iter().position(|&p| p == a).unwrap();
                    let i_b = programs.iter().position(|&p| p == b).unwrap();

                    programs.swap(i_a, i_b);
                },
            }
        }
    }

    pub fn solve_a(moves: &[DanceMove]) -> String {
        let mut programs = ('a'..='p').collect();

        for m in moves {
            m.execute(&mut programs);
        }

        programs.into_iter().collect()
    }

    #[cfg(test)]
    mod test_day16 {
        use super::*;

        #[test]
        fn test_dance_moves() {
            let mut programs = ('a'..='e').collect();

            DanceMove::Spin(1).execute(&mut programs);
            assert_eq!(programs.iter().collect::<String>(), "eabcd");

            DanceMove::Exchange(3, 4).execute(&mut programs);
            assert_eq!(programs.iter().collect::<String>(), "eabdc");

            DanceMove::Partner('e', 'b').execute(&mut programs);
            assert_eq!(programs.iter().collect::<String>(), "baedc");
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let moves = stdin.lock().lines().next().unwrap().unwrap().split(',')
        .map(|m| {
            let kind = m.chars().next().unwrap();
            let params = m.chars().skip(1).collect::<String>()
                .split('/').map(|s| String::from(s)).collect::<Vec<String>>();

            match kind {
                's' => { 
                    day16::DanceMove::Spin(params[0].parse().unwrap())
                },
                'x' => {
                    day16::DanceMove::Exchange(
                        params[0].parse().unwrap(),
                        params[1].parse().unwrap(),
                        )
                },
                'p' => {
                    day16::DanceMove::Partner(
                        params[0].chars().next().unwrap(),
                        params[1].chars().next().unwrap(),
                        )
                },
                _ => unreachable!(),
            }
        }).collect::<Vec<_>>();


    println!("Solution A-part: {}", day16::solve_a(&moves));

}
