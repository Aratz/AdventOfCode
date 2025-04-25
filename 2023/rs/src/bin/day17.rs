mod day17 {
    use std::cmp::{Ordering, max, Reverse};
    use std::collections::{BinaryHeap, HashSet};

    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    struct State {
        pos: (i32, i32),
        dir: (i32, i32),
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.pos.cmp(&self.pos)
        }
    }

    fn parse(input: &str) -> Vec<Vec<u32>> {
        input.lines().map(|l|
            l.chars().map(|c| c.to_digit(10).unwrap()).collect()
        ).collect()
    }

    fn solve(input: &str, min_mv: i32, max_mv: i32) -> u32 {
        let grid = parse(input);
        let (h, w) = (grid.len() as i32, grid[0].len() as i32);

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, State { pos: (0, 0), dir: (1, 0) })));
        heap.push(Reverse((0, State { pos: (0, 0), dir: (0, 1) })));
        let mut visited = HashSet::new();

        while let Some(Reverse((loss, state))) = heap.pop() {
            if visited.contains(&state) { continue; }

            if state.pos == (h - 1, w - 1) { return loss; }

            visited.insert(state);

            for dpos in (-max_mv..=max_mv).filter(|&v| v.abs() >= min_mv)
                    .map(|v| if state.dir.0 == 0 { (v, 0) } else { (0, v) }) {
                let dest = (state.pos.0 + dpos.0, state.pos.1 + dpos.1);
                if !(0 <= dest.0 && dest.0 < h && 0 <= dest.1 && dest.1 < w) { continue; }

                let n = max(dpos.0.abs(), dpos.1.abs());
                let dir = (dpos.0 / n, dpos.1 / n);
                let dloss: u32 = (1..=n)
                    .map(|i| (state.pos.0 + i*dir.0, state.pos.1 + i*dir.1))
                    .map(|(x, y)| grid[x as usize][y as usize])
                    .sum();
                heap.push(Reverse((loss + dloss, State { pos: dest, dir: dir })));
            }
        }

        unreachable!();
    }

    pub fn solve_a(input: &str) -> u32 {
        solve(input, 1, 3)
    }

    pub fn solve_b(input: &str) -> u32 {
        solve(input, 4, 10)
    }

    #[cfg(test)]
    mod test_day17 {
        use super::*;

        static INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 102);
        }

        #[test]
        fn test_solve_b() {
            let input2 = "111111111111
999999999991
999999999991
999999999991
999999999991";
            assert_eq!(solve_b(input2), 71);
            assert_eq!(solve_b(INPUT), 94);
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

    println!("Solution A-part: {}", day17::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day17::solve_b(&buffer.trim()));
}
