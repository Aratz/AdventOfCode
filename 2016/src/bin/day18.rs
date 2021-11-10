mod day18 {
    pub fn solve_ab(start_row: &str, max_row: usize) -> usize {
        let mut tiles: Vec<char> = format!(".{}.", start_row).chars().collect();
        let mut res = tiles[1..=start_row.len()].iter().filter(|&&c| c == '.').count();

        for _ in 1..max_row {
            tiles = (0..=start_row.len()+1)
                .map(|i| match i {
                     0 => '.',
                     len if len == start_row.len() + 1 => '.',
                     _ => if tiles[i-1] == tiles[i] && tiles[i] != tiles[i+1]
                          || tiles[i-1] != tiles[i] && tiles[i] == tiles[i+1]
                            { '^' } else { '.' },
                    }).collect();
            res += tiles[1..=start_row.len()].iter().filter(|&&c| c == '.').count()
        }

        res
    }

    #[cfg(test)]
    mod test_day18 {
        use super::*;

        #[test]
        fn test_solve_ab() {
            assert_eq!(solve_ab("..^^.", 3), 6);
            assert_eq!(solve_ab(".^^.^.^^^^", 10), 38);
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

    println!("Solution A-part: {}", day18::solve_ab(&buffer.trim(), 40));
    println!("Solution B-part: {}", day18::solve_ab(&buffer.trim(), 400_000));
}
