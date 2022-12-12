mod day12 {
    use std::collections::{VecDeque, HashMap};

    fn parse(input: &str) -> (Vec<Vec<i32>>, ((usize, usize), (usize, usize))) {
        let mut carta: Vec<Vec<i32>> = input.lines()
            .map(|l| l.chars()
                 .map(|c| c as i32)
                 .collect())
            .collect();
        let s = carta.iter().enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, h)| ((i, j), h)))
            .find(|((_i, _j), &h)| h == 'S' as i32).unwrap().0;
        let e = carta.iter().enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, h)| ((i, j), h)))
            .find(|((_i, _j), &h)| h == 'E' as i32).unwrap().0;
        carta[s.0][s.1] = 'a' as i32;
        carta[e.0][e.1] = 'z' as i32;

        (carta, (s, e))
    }

    #[inline]
    fn get_neighbors(pos: (usize, usize), carta: &[Vec<i32>]) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if pos.0 > 0 { neighbors.push((pos.0 - 1, pos.1)); }
        if pos.1 > 0 { neighbors.push((pos.0, pos.1 - 1)); }
        if pos.0 < carta.len() - 1 { neighbors.push((pos.0 + 1, pos.1)); }
        if pos.1 < carta[0].len() - 1 { neighbors.push((pos.0, pos.1 + 1)); }

        neighbors
    }

    pub fn solve_a(input: &str) -> i32 {
        let (carta, (s, e)) = parse(input);

        let mut queue = VecDeque::from([(s, 0)]);
        let mut dists = HashMap::from([(s, 0)]);

        while let Some((pos, dist)) = queue.pop_front() {
            if pos == e {
                return dist;
            }

            let (i_p, j_p) = pos;

            for (i_n, j_n) in get_neighbors(pos, &carta).into_iter() {
                if carta[i_n][j_n] <= carta[i_p][j_p] + 1
                        && !dists.contains_key(&(i_n, j_n)) {
                    queue.push_back(((i_n, j_n), dist + 1));
                    dists.insert((i_n, j_n), dist + 1);
                }
            }
        }
        unreachable!();
    }

    pub fn solve_b(input: &str) -> i32 {
        let (carta, (_, e)) = parse(input);

        let mut queue = VecDeque::from([(e, 0)]);
        let mut dists = HashMap::from([(e, 0)]);

        while let Some((pos, dist)) = queue.pop_front() {
            let (i_p, j_p) = pos;
            if carta[i_p][j_p] == 'a' as i32 {
                return dist;
            }

            for (i_n, j_n) in get_neighbors(pos, &carta).into_iter() {
                if carta[i_p][j_p] <= carta[i_n][j_n] + 1
                        && !dists.contains_key(&(i_n, j_n)) {
                    queue.push_back(((i_n, j_n), dist + 1));
                    dists.insert((i_n, j_n), dist + 1);
                }
            }
        }
        unreachable!();
    }

    #[cfg(test)]
    mod test_day12 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
            assert_eq!(solve_a(input), 31);
        }

        #[test]
        fn test_solve_b() {
            let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
            assert_eq!(solve_b(input), 29);
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

    println!("Solution A-part: {}", day12::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day12::solve_b(&buffer.trim()));
}
