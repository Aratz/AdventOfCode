mod day25 {
    fn step(seafloor: &[Vec<char>]) -> Vec<Vec<char>> {
        let max_i = seafloor.len();
        let max_j = seafloor[0].len();

        let mut new_seafloor= seafloor.to_vec();

        for (di, dj) in vec![(0, 1), (1, 0)] {
            let herd = if (di, dj) == (0, 1) { '>' } else { 'v' };

            let moveable: Vec<_> = new_seafloor.iter().enumerate()
                .flat_map(|(i, row)| row.iter().enumerate()
                          .filter_map(move |(j, ccbr)| if *ccbr == herd { Some((i, j)) } else { None }))
                .filter(|&(i, j)| new_seafloor[(i + di)%max_i][(j + dj)%max_j] == '.')
                .collect();

            for (i, j) in moveable {
                new_seafloor[i][j] = '.';
                new_seafloor[(i + di)%max_i][(j + dj)%max_j] = herd;
            }
        }

        new_seafloor
    }

    pub fn solve(seafloor: &[String]) -> usize {
        let mut seafloor: Vec<_> = seafloor.iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();

        for i in 1.. {
            let new_seafloor = step(&seafloor);

            if new_seafloor == seafloor {
                return i;
            }

            seafloor = new_seafloor;
        }

        unreachable!();
    }

    #[cfg(test)]
    mod test_day25 {
        use super::*;

        #[test]
        fn test_step() {
            let input = vec!["...>>>>>...".chars().collect()];

            assert_eq!(step(&input)[0], "...>>>>.>..".chars().collect::<Vec<_>>());


            let input: Vec<_> = "..........
.>v....v..
.......>..
..........".lines().map(|s| s.chars().collect::<Vec<char>>()).collect();

            let output: Vec<_> = "..........
.>........
..v....v>.
..........
".lines().map(|s| s.chars().collect::<Vec<char>>()).collect();

            assert_eq!(step(&input), output);
        }

        #[test]
        fn test_solve() {
            let input: Vec<_> = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>".lines().map(|s| s.to_string()).collect();

            assert_eq!(solve(&input), 58);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let input: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    println!("Solution: {}", day25::solve(&input));
}
