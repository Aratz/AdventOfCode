mod day19 {
    enum Dir {
        Up, Left, Down, Right,
    }

    pub fn solve_ab(tubes: &[Vec<char>]) -> (String, usize) {
        let mut i = 0;
        let mut j = tubes[i].iter().position(|&c| c == '|').unwrap();
        let mut dir = Dir::Down;
        let mut res = String::new();
        let mut step_count = 0;

        loop {
            match tubes[i][j] {
                'A'..='Z' => {
                    res.push(tubes[i][j]);
                },
                '|' | '-' => {},
                '+' => {
                    match dir {
                        Dir::Up | Dir::Down => {
                            match tubes[i][j - 1] {
                                '-' | 'A'..='Z' => { dir = Dir::Left; },
                                _ => { dir = Dir::Right; },
                            }
                        },
                        Dir::Left | Dir::Right => {
                            match tubes[i - 1][j] {
                                '|' | 'A'..='Z' => { dir = Dir::Up; },
                                _ => { dir = Dir::Down; },
                            }
                        },
                    }
                },
                _ => {
                    return (res, step_count);
                },
            }

            match dir {
                Dir::Up    => { i -= 1; },
                Dir::Left  => { j -= 1; },
                Dir::Down  => { i += 1; },
                Dir::Right => { j += 1;},
            }
            step_count += 1;
        }
    }

    #[cfg(test)]
    mod test_day19 {
        use super::*;

        #[test]
        fn test_solve_ab() {
            let tubes: Vec<Vec<char>> = "     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ \n                \n".lines()
                .map(|l| l.chars().collect())
                .collect();

            let (res, step_count) = solve_ab(&tubes);
            assert_eq!(res.as_str(), "ABCDEF");
            assert_eq!(step_count, 38);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let tubes: Vec<Vec<char>> = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect()).collect();

    let (res, step_count) = day19::solve_ab(&tubes);
    println!("Solution A-part: {}", res);
    println!("Solution B-part: {}", step_count);
}
