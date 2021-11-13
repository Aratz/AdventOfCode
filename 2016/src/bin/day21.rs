mod day21 {
    use std::collections::HashMap;

    pub fn solve_ab(ops: &[String], backtrack: bool) -> String {
        let mut password: Vec<char> = if !backtrack { "abcdefgh".chars().collect() } else { "fbgdceah".chars().collect() };

        let rotbased: HashMap<usize, usize> = vec![
            (1, 1), (3, 2), (5, 3), (7, 4),
            (2, 6), (4, 7), (6, 0), (0, 1)].into_iter().collect();
        let mut ops = ops.to_vec();
        if backtrack {
            ops.reverse();
        }

        for op in ops {
            let words: Vec<String> = op.split(" ")
                .map(|s| s.into()).collect();
            match words[0].as_str() {
                "swap" => {
                    match words[1].as_str() {
                        "position" => {
                            password.swap(
                                words[2].parse().unwrap(),
                                words[5].parse().unwrap());
                        },
                        "letter" => {
                            let ix = password.iter().enumerate()
                                .find(|(_i, c)| c.to_string() == words[2]).unwrap().0;
                            let iy = password.iter().enumerate()
                                .find(|(_i, c)| c.to_string() == words[5]).unwrap().0;
                            password.swap(ix, iy);
                        },
                        _ => unreachable!(),
                    }
                },
                "rotate" => {
                    match words[1].as_str() {
                        "left" => {
                            if !backtrack {
                                password.rotate_left(words[2].parse().unwrap());
                            }
                            else {
                                password.rotate_right(words[2].parse().unwrap());
                            }
                        },
                        "right" => {
                            if !backtrack {
                                password.rotate_right(words[2].parse().unwrap());
                            }
                            else {
                                password.rotate_left(words[2].parse().unwrap());
                            }
                        },
                        "based" => {
                            if !backtrack {
                                let ix = password.iter().enumerate()
                                    .find(|(_i, c)| c.to_string() == words[6]).unwrap().0;
                                password.rotate_right(1);
                                password.rotate_right(ix);
                                    if ix >= 4 {
                                        password.rotate_right(1); 
                                    };
                            }
                            else {
                                let ix = password.iter().enumerate()
                                    .find(|(_i, c)| c.to_string() == words[6]).unwrap().0;
                                let rot = rotbased[&ix];
                                password.rotate_left(rot);
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                "reverse" => {
                    let mut ix = words[2].parse().unwrap();
                    let mut iy = words[4].parse().unwrap();

                    while ix < iy {
                        password.swap(ix, iy);
                        ix += 1;
                        iy -= 1;
                    }
                },
                "move" => {
                    if !backtrack {
                        let x = password.remove(words[2].parse().unwrap());
                        password.insert(words[5].parse().unwrap(), x);
                    }
                    else {
                        let x = password.remove(words[5].parse().unwrap());
                        password.insert(words[2].parse().unwrap(), x);
                    }
                },
                _ => unreachable!(),
            }
        }

        password.iter().collect()
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let ops: Vec<String> = stdin.lock().lines()
        .map(|l| l.unwrap()).collect();

    println!("Solution A-part: {}", day21::solve_ab(&ops, false));
    println!("Solution B-part: {}", day21::solve_ab(&ops, true));
}
