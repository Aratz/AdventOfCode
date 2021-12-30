mod day21 {
    use std::collections::HashMap;

    type PossibleUniverse= HashMap<((usize, usize), (usize, usize)), usize>;

    fn get_i(player: usize, start: usize, total_score: usize) -> (usize, usize) {
        let offset = match player {
            1 => 2,
            2 => 5,
            _ => unreachable!(),
        };

        let (i, score) = (0..).map(|i| (3*(6*i + offset)))
            .scan(start - 1, |acc, x| {
                *acc = (*acc + x)%10;
                Some(*acc)
            })
            .scan(0, |acc, x| {
                *acc = *acc + x + 1;
                Some(*acc)
            })
            .enumerate()
            .find(|&(_i, score)| { score >= total_score}).unwrap();
        (i + 1, score)
    }

    fn get_score_at_i(i: usize, player: usize, start: usize) -> usize {
        let offset = match player {
            1 => 2,
            2 => 5,
            _ => unreachable!(),
        };

        (0..i).map(|i| (3*(6*i + offset)))
            .scan(start - 1, |acc, x| {
                *acc = (*acc + x)%10;
                Some(*acc)
            })
        .fold(0, |acc, x| acc + x + 1)
    }

    pub fn solve_a(start: (usize, usize)) -> usize {
        let p1_win = get_i(1, start.0, 1000);
        let p2_win = get_i(2, start.1, 1000);


        if p1_win.0 <= p2_win.0 {
            let i_win = p1_win.0;
            let score_lose = get_score_at_i(i_win - 1, 2, start.1);
            (6*(i_win - 1) + 3)*score_lose
        }
        else {
            let i_win = p2_win.0;
            let score_lose = get_score_at_i(i_win, 1, start.0);

            6*i_win*score_lose
        }
    }

    fn compute_next_round( dp: PossibleUniverse, player: usize)
        -> PossibleUniverse {
        let mut new_dp = HashMap::new();


        for (((pos1, score1), (pos2, score2)), n_uni) in dp {
            if score1 >= 21 || score2 >= 21 {
                *new_dp.entry(((pos1, score1), (pos2, score2))).or_default() += n_uni;
                continue;
            }

            for throw1 in 1..=3 {
                for throw2 in 1..=3 {
                    for throw3 in 1..=3 {
                        match player {
                            1 => {
                                let pos1 = (pos1 - 1 + throw1 + throw2 + throw3)%10 + 1;
                                let score1 = score1 + pos1;
                                *new_dp.entry(((pos1, score1), (pos2, score2))).or_default() += n_uni;

                            },
                            2 => {
                                let pos2 = (pos2 - 1 + throw1 + throw2 + throw3)%10 + 1;
                                let score2 = score2 + pos2;
                                *new_dp.entry(((pos1, score1), (pos2, score2))).or_default() += n_uni;
                            },
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        new_dp
    }

    pub fn solve_b(start: (usize, usize)) -> usize {
        let mut dp: HashMap<_, _> = vec![(((start.0, 0), (start.1, 0)), 1)].into_iter().collect();
        let mut current_player = 1;

        while dp.keys().any(|&((_pos1, score1), (_pos2, score2))| score1 < 21 && score2 < 21) {
            dp = compute_next_round(dp, current_player);
            current_player = (1 - (current_player - 1)) + 1;
        }


        let n_uni_final = dp.iter().map(
            |(((_pos1, score1), (_pos2, score2)), n_uni)|
            if score1 > score2 { (*n_uni, 0) } else { (0, *n_uni) })
            .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));


        if n_uni_final.0 > n_uni_final.1 {
            n_uni_final.0
        }
        else {
            n_uni_final.1
        }
    }

    #[cfg(test)]
    mod test_day21 {
        use super::*;

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a((4, 8)), 739785);
        }

        #[test]
        fn test_get_i() {
            assert_eq!(get_i(1, 4, 10), (1, 10));
            assert_eq!(get_i(1, 4, 14), (2, 14));
            assert_eq!(get_i(1, 4, 25), (4, 26));

            assert_eq!(get_i(2, 8, 10), (3, 16));
            //6* 3 = 18
        }

        #[test]
        fn test_get_score() {
            assert_eq!(get_score_at_i(1, 1, 4), 10);
            assert_eq!(get_score_at_i(2, 1, 4), 14);

            assert_eq!(get_score_at_i(1, 2, 8), 3);
            assert_eq!(get_score_at_i(2, 2, 8), 9);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b((4, 8)), 444356092776315);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let p1 = lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap() as usize;
    let p2 = lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap() as usize;

    println!("Solution A-part: {}", day21::solve_a((p1, p2)));
    println!("Solution B-part: {}", day21::solve_b((p1, p2)));
}
