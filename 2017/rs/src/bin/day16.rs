mod day16 {
    use std::mem;

    #[derive(Copy, Clone, Debug)]
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

    fn exec_dance(
            programs: &mut Vec<char>,
            moves: &[DanceMove]) {
        for m in moves {
            m.execute(programs);
        }
    }

    fn exec_dance_fast(
            programs: &mut Vec<char>,
            moves: &[DanceMove],
            mut n: usize) {

        let mut i_prog: Vec<usize> = (0..programs.len()).collect();
        let mut dance = programs.clone();
        exec_dance(&mut dance, moves);

        let mut i_dance: Vec<usize> = dance.iter().map(
            |new_pos| programs.iter().position(|c| c == new_pos).unwrap()).collect();

        while n > 0 {
            if n % 2 == 1 {
                i_prog = i_dance.iter().map(|&p| i_prog[p]).collect();
            }

            i_dance = i_dance.iter().map(|&d| i_dance[d]).collect();
            n /= 2;
        }

        *programs = i_prog.iter().map(|&p| programs[p]).collect();
    }

    pub fn solve_ab(moves: &[DanceMove], n: usize) -> String {
        let mut programs: Vec<char> = ('a'..='p').collect();

        let exspin = moves.iter().filter(
            |&m| mem::discriminant(m) ==  mem::discriminant(&DanceMove::Spin(0))
            || mem::discriminant(m) ==  mem::discriminant(&DanceMove::Exchange(0, 1))
            ).copied().collect::<Vec<DanceMove>>();

        let partner = moves.iter().filter(
            |&m| mem::discriminant(m) ==  mem::discriminant(&DanceMove::Partner('a', 'b'))
            ).copied().collect::<Vec<DanceMove>>();

        exec_dance_fast(&mut programs, &exspin, n);
        exec_dance_fast(&mut programs, &partner, n);

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

        #[test]
        fn test_solve_a() {
            let moves = vec![
                DanceMove::Spin(1),
                DanceMove::Exchange(3, 4),
                DanceMove::Partner('e', 'b'),
            ];

            let mut programs: Vec<_> = ('a'..='p').collect();
            exec_dance(&mut programs, &moves);

            assert_eq!(solve_ab(&moves, 1), programs.into_iter().collect::<String>());
        }

        #[test]
        fn test_solve_b() {
            let moves = vec![
                DanceMove::Spin(1),
                DanceMove::Exchange(3, 4),
                DanceMove::Partner('e', 'b'),
            ];

            let n = 5;


            let mut programs: Vec<_> = ('a'..='p').collect();
            for _ in 0..n {
                exec_dance(&mut programs, &moves);
            }

            assert_eq!(solve_ab(&moves, 5), programs.into_iter().collect::<String>());
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


    println!("Solution A-part: {}", day16::solve_ab(&moves, 1));
    println!("Solution B-part: {}", day16::solve_ab(&moves, 1_000_000_000));
}
