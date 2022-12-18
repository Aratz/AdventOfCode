mod day18 {
    use std::collections::{HashSet, VecDeque};

    fn parse(input: &str) -> HashSet<(i32, i32, i32)> {
        input.lines()
            .map(|row| {
                let lava_bit: Vec<i32> = row.split(',')
                    .map(|v| v.parse().unwrap())
                    .collect();
                (lava_bit[0], lava_bit[1], lava_bit[2])
            })
            .collect()
    }
    pub fn solve_a(input: &str) -> usize {
        let lava_bits = parse(input);
        let neighbors = vec![
            (1, 0, 0), (-1, 0, 0),
            (0, 1, 0), (0, -1, 0),
            (0, 0, 1), (0, 0, -1)];

        lava_bits.iter()
            .flat_map(|(x, y, z)| neighbors.iter()
                      .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz)))
            .filter(|lb| !lava_bits.contains(&lb))
            .count()
    }

    pub fn solve_b(input: &str) -> usize {
        let lava_bits = parse(input);

        let mut outside = HashSet::new();
        let mut queue = VecDeque::from(vec![(0, 0, 0)]);

        let (xmax, ymax, zmax) = (
            lava_bits.iter().map(|(x, _, _)| x).max().unwrap() + 1,
            lava_bits.iter().map(|(_, y, _)| y).max().unwrap() + 1,
            lava_bits.iter().map(|(_, _, z)| z).max().unwrap() + 1,
        );

        let (xmin, ymin, zmin) = (
            lava_bits.iter().map(|(x, _, _)| x).min().unwrap() - 1,
            lava_bits.iter().map(|(_, y, _)| y).min().unwrap() - 1,
            lava_bits.iter().map(|(_, _, z)| z).min().unwrap() - 1,
        );

        let neighbors = vec![
            (1, 0, 0), (-1, 0, 0),
            (0, 1, 0), (0, -1, 0),
            (0, 0, 1), (0, 0, -1)];

        while let Some(cube) = queue.pop_front() {
            if outside.contains(&cube) { continue; }

            outside.insert(cube);

            let (x, y, z) = cube;
            for (dx, dy, dz) in &neighbors {
                let new_cube = (x + dx, y + dy, z + dz);
                if xmin <= new_cube.0 && new_cube.0 <= xmax
                    && ymin <= new_cube.1 && new_cube.1 <= ymax
                    && zmin <= new_cube.2 && new_cube.2 <= zmax
                    && !lava_bits.contains(&new_cube) {
                    queue.push_back(new_cube);
                }
            }
        }

        lava_bits.iter()
            .flat_map(|(x, y, z)| neighbors.iter()
                      .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz)))
            .filter(|lb| outside.contains(&lb))
            .count()
    }

    #[cfg(test)]
    mod test_day18 {
        use super::*;

        static INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 64);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 58);
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

    println!("Solution A-part: {}", day18::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day18::solve_b(&buffer.trim()));
}
