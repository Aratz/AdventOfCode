mod day23 {
    use std::cmp::{min, max};
    use std::collections::{HashSet, HashMap};

    const NEIGHBOUR_LOOKUP: [([(i32, i32); 3], (i32, i32)); 4] = [
        ([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
        ([(1, -1), (1, 0), (1, 1)], (1, 0)),
        ([(-1, -1), (0, -1), (1, -1)], (0, -1)),
        ([(-1, 1), (0, 1), (1, 1)], (0, 1)),
    ];

    #[inline]
    fn parse(input: &str) -> HashSet<(i32, i32)> {
        input.lines().enumerate()
            .flat_map(
                |(i, l)| 
                l.chars().enumerate()
                .filter(|&(_j, c)| c == '#')
                .map(move |(j, _c)| (i, j))
            ).map(|(i, j)| (i as i32, j as i32))
            .collect()
    }

    fn compute_round(mut elves: HashSet<(i32, i32)>, next_lookup: usize)
            -> HashSet<(i32, i32)> {

        let mut proposals = HashMap::new();
        let mut new_pos = HashSet::new();
        for (i, j) in elves.iter() {
            if !(0..4)
                .map(|n| NEIGHBOUR_LOOKUP[n % 4])
                .all(|(ngh, _dir)| ngh.iter()
                    .all(|(di, dj)| !elves.contains(&(i + di, j + dj)))) {
                if let Some((_ngh, (di, dj))) = (next_lookup..next_lookup+4)
                    .map(|n| NEIGHBOUR_LOOKUP[n % 4])
                    .find(
                        |(ngh, _dir)| ngh.iter()
                        .all(|(di, dj)| !elves.contains(&(i + di, j + dj)))) {
                    let tentative_move = (i + di, j + dj);
                    proposals.entry(tentative_move)
                        .and_modify(|e| { *e = None; })
                        .or_insert(Some((*i, *j)));
                }
            }
        }

        for (dest, src) in proposals.into_iter()
                .filter(|(_dest, src)| src.is_some()) {
            let src = src.unwrap();
            new_pos.insert(dest);
            elves.remove(&src);
        }

        elves.union(&new_pos).cloned().collect()
    }

    pub fn solve_a(input: &str) -> i32 {
        let max_round = 10;

        let mut elves = parse(input);

        let mut next_lookup = 0;

        for _ in 0..max_round {
            elves = compute_round(elves, next_lookup);
            next_lookup += 1;
        }

        let elve_count = elves.len() as i32;
        let ((min_i, max_i), (min_j, max_j)) = elves.into_iter().fold(
            ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
            |((min_i, max_i), (min_j, max_j)), (i, j)| (
                (min(min_i, i), max(max_i, i)),
                (min(min_j, j), max(max_j, j))
        ));

        (max_i - min_i + 1) * (max_j - min_j + 1) - elve_count
    }

    pub fn solve_b(input: &str) -> i32 {
        let mut elves = parse(input);

        let mut next_lookup = 0;
        let mut n_round = 0;

        loop {
            n_round += 1;
            let new_elves = compute_round(elves.clone(), next_lookup);

            if new_elves == elves { break; }
            else { elves = new_elves; }

            next_lookup += 1;
        }

        n_round
    }

    #[cfg(test)]
    mod test_day23 {
        use super::*;

        static INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 110);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 20);
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

    println!("Solution A-part: {}", day23::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day23::solve_b(&buffer.trim()));
}
