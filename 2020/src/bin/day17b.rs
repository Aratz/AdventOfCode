mod day17b {
    use std::collections::HashSet;
    use std::str::FromStr;

    pub struct Grid(HashSet<(i32, i32, i32, i32)>);

    impl Grid {
        pub fn update(&mut self) {
            self.0 = Grid(
                self.0.iter()
                .flat_map(|(x, y, z, w)|
                    (-1..=1).flat_map(
                        move |dx| (-1..=1).flat_map(
                            move |dy| (-1..=1).flat_map(
                                move |dz| (-1..=1).map(
                                    move |dw| (x + dx, y + dy, z + dz, w + dw)
                                )
                            )
                        )
                    )
                ).filter(|coords| {
                    let n = self.n_neighbors(coords);
                    n == 3 || (self.0.contains(coords) && n == 2)
                }).collect()
                ).0;
        }

        pub fn count(&self) -> usize {
            self.0.len()
        }

        fn n_neighbors(&self, coords: &(i32, i32, i32, i32)) -> usize {
            let (x, y, z, w) = coords;

            (-1..=1).flat_map(
                move |dx| (-1..=1).flat_map(
                    move |dy| (-1..=1).flat_map(
                        move |dz| (-1..=1)
                            .filter(move |&dw| dx != 0 || dy != 0 || dz != 0 || dw != 0)
                            .map(move |dw| (x + dx, y + dy, z + dz, w + dw))
                        )
                    )
                )
                .filter(|coords| self.0.contains(coords))
                .count()
        }
    }

    impl FromStr for Grid {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Grid(s.split("\n").zip(0..)
                .flat_map(
                    |(row, i)| row.chars().zip(0..)
                        .filter(|(c, _j)| *c=='#')
                        .map(move |(_c, j)| (i, j, 0, 0))
                ).collect()))
        }
    }

    #[cfg(test)]
    mod test_day17b {
        use super::*;

        #[test]
        fn test_n_neighbors() {
            let grid: Grid = ".#.\n..#\n###\n".parse().unwrap();
            let mut neighbors: Vec<usize> = grid.0.iter().map(|coords| grid.n_neighbors(coords)).collect();

            neighbors.sort();
            assert_eq!(neighbors, vec![1, 1, 2, 3, 3]);
        }

        #[test]
        fn test_solve_a() {
            let mut grid: Grid = ".#.\n..#\n###\n".parse().unwrap();
            for _ in 0..6 {
                grid.update();
            }

            assert_eq!(grid.count(), 848);
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

    let mut grid: day17b::Grid = buffer.parse().unwrap();
    for _ in 0..6 {
        grid.update();
    }

    println!("Solution A-part: {}", grid.count());
}
