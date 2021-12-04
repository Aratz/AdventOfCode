mod day04 {
    #[derive(Clone)]
    pub struct BingoBoard {
        board: Vec<Vec<i32>>,
        mask: Vec<Vec<bool>>,
        won: bool
    }

    impl BingoBoard {
        pub fn new(board: Vec<Vec<i32>>) -> Self {
            let mask = (0..5).map(|_| (0..5).map(|_| false).collect()).collect();
            let won = false;

            Self { board, mask, won }
        }

        fn fill(&mut self, n_draw: i32) -> Option<i32> {
            //find pos & mark
            if self.won { return None; }

            if let Some(((i, j), _n_board)) = self.board.iter().enumerate()
                    .flat_map(|(i, row)| row.iter().enumerate()
                              .map(move |(j, n_board)| ((i, j), n_board)))
                    .find(|((_i, _j), n_board)| **n_board == n_draw) {

                self.mask[i][j] = true;

                if self.mask[i].iter().all(|&v| v)
                        || self.mask.iter().map(|row| row[j]).all(|v| v) {
                    
                    let sum_unmarked: i32 = self.board.iter().enumerate()
                        .flat_map(|(i, row)| row.iter().enumerate()
                                  .map(move |(j, n_board)| ((i, j), n_board)))
                        .filter(|((i, j), _n_board)| !self.mask[*i][*j])
                        .map(|((_i, _j), n_board)| n_board)
                        .sum();
                    self.won = true;
                    return Some(sum_unmarked * n_draw);
                }
            }

            None
        }
    }

    pub fn solve_a(draw: &[i32], mut bingo_boards: Vec<BingoBoard>) -> Result<i32, &'static str> {
        for &n_draw in draw {
            for bingoboard in bingo_boards.iter_mut() {
                if let Some(res) = bingoboard.fill(n_draw) {
                    return Ok(res);
                }
            }
        }

        Err("No board won")
    }

    pub fn solve_b(draw: &[i32], mut bingo_boards: Vec<BingoBoard>) -> Result<i32, &'static str> {
        let mut last_to_win = Err("No board won");
        for &n_draw in draw {
            for bingoboard in bingo_boards.iter_mut() {
                if let Some(res) = bingoboard.fill(n_draw) {
                    last_to_win = Ok(res);
                }
            }
        }

        last_to_win
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<String> = stdin.lock().lines()
        .map(|s| s.unwrap())
        .collect();

    let mut lines_iter = lines.iter();

    let draw: Vec<i32> = lines_iter.next().unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut bingo_boards: Vec<day04::BingoBoard> = vec![];

    while lines_iter.next().is_some() {
        let board: Vec<Vec<i32>> = (0..5).map(|_| lines_iter.next().unwrap())
            .map(|s| s.split(" ")
                 .filter(|s| !s.is_empty())
                 .map(|n| n.parse().unwrap()).collect())
            .collect();
        bingo_boards.push(day04::BingoBoard::new(board));
    }

    println!("Solution A-part: {}", day04::solve_a(&draw, bingo_boards.clone()).unwrap());
    println!("Solution B-part: {}", day04::solve_b(&draw, bingo_boards.clone()).unwrap());
}
