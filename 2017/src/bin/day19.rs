mod day19 {
    enum Dir {
        Up, Left, Down, Right,
    }

    pub fn solve_a(tubes: &[Vec<char>]) -> String {
        let mut i = 0;
        let mut j = tubes[i].iter().position(|&c| c == '|').unwrap();
        let mut dir = Dir::Down;
        let mut res = String::new();

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
                    return res;
                },
            }

            match dir {
                Dir::Up    => { i -= 1; },
                Dir::Left  => { j -= 1; },
                Dir::Down  => { i += 1; },
                Dir::Right => { j += 1;},
            }
        }
    }

    #[cfg(test)]
    mod test_day19 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let tubes: Vec<Vec<char>> = "     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ \n                \n".lines()
                .map(|l| l.chars().collect())
                .collect();

            assert_eq!(solve_a(&tubes).as_str(), "ABCDEF");
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let tubes: Vec<Vec<char>> = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect()).collect();

    println!("Solution A-part: {}", day19::solve_a(&tubes));
}
