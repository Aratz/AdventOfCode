mod day10 {
    use std::collections::{VecDeque, HashSet};

    fn parse(input: &str) -> Vec<Vec<char>> {
        input.lines()
            .map(|l| l.chars().collect())
            .collect()
    }

    fn mark_pipes(map: &mut [Vec<char>]) -> (usize, HashSet<(usize, usize)>) {
        let (i, j) = map.iter().enumerate()
            .find_map(|(i, row)|
                  if let Some((j, _c)) = row.iter().enumerate()
                          .find(|(_j, c)| c == &&'S') {
                    Some((i, j))
                  }
                  else { None }
            ).unwrap();

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut max_d = 0;

        visited.insert((i, j));

        let mut subs_s: HashSet<char> = vec!['-', '|', 'F', '7', 'L', 'J'].into_iter().collect();
        if i > 0 && vec!['F', '|', '7'].contains(&map[i-1][j]) {
            queue.push_back(((i - 1, j), 1));
            subs_s = subs_s.intersection(&vec!['J', '|', 'L'].into_iter().collect())
                .cloned().collect();
        }
        if i + 1 < map.len() && vec!['J', '|', 'L'].contains(&map[i+1][j]) {
            queue.push_back(((i + 1, j), 1));
            subs_s = subs_s.intersection(&vec!['F', '|', '7'].into_iter().collect())
                .cloned().collect();
        }
        if j > 0 && vec!['F', '-', 'L'].contains(&map[i][j-1]) {
            queue.push_back(((i, j - 1), 1));
            subs_s = subs_s.intersection(&vec!['J', '-', '7'].into_iter().collect())
                .cloned().collect();
        }
        if j + 1 < map[0].len() && vec!['J', '-', '7'].contains(&map[i][j+1]){
            queue.push_back(((i, j + 1), 1));
            subs_s = subs_s.intersection(&vec!['F', '-', 'L'].into_iter().collect())
                .cloned().collect();
        }

        map[i][j] = subs_s.into_iter().next().unwrap();

        while let Some(((i, j), d)) = queue.pop_front() {
            visited.insert((i, j));
            match map[i][j] {
                '|' => {
                    if i > 0 && !visited.contains(&(i - 1, j)) {
                        queue.push_back(((i - 1, j), d + 1))
                    }
                    else if i + 1 < map.len() && !visited.contains(&(i + 1, j)) {
                        queue.push_back(((i + 1, j), d + 1))
                    }
                },
                'F' => {
                    if j + 1 < map[0].len() && !visited.contains(&(i, j + 1)) {
                        queue.push_back(((i, j + 1), d + 1))
                    }
                    else if i + 1 < map.len() && !visited.contains(&(i + 1, j)) {
                        queue.push_back(((i + 1, j), d + 1))
                    }
                },
                '7' => {
                    if j > 0 && !visited.contains(&(i, j - 1)) {
                        queue.push_back(((i, j - 1), d + 1))
                    }
                    else if i + 1 < map.len() && !visited.contains(&(i + 1, j)) {
                        queue.push_back(((i + 1, j), d + 1))
                    }
                },
                'J' => {
                    if i > 0 && !visited.contains(&(i - 1, j)) {
                        queue.push_back(((i - 1, j), d + 1))
                    }
                    else if j > 0 && !visited.contains(&(i, j - 1)) {
                        queue.push_back(((i, j - 1), d + 1))
                    }
                },
                'L' => {
                    if i > 0 && !visited.contains(&(i - 1, j)) {
                        queue.push_back(((i - 1, j), d + 1))
                    }
                    else if j + 1 < map[0].len() && !visited.contains(&(i, j + 1)) {
                        queue.push_back(((i, j + 1), d + 1))
                    }
                },
                '-' => {
                    if j > 0 && !visited.contains(&(i, j - 1)) {
                        queue.push_back(((i, j - 1), d + 1))
                    }
                    else if j + 1 < map[0].len() && !visited.contains(&(i, j + 1)) {
                        queue.push_back(((i, j + 1), d + 1))
                    }
                },
                _ => unreachable!(),
            }

            max_d = d;
        }

        (max_d, visited)
    }

    fn count_inside(map: &[Vec<char>], pipe: &HashSet<(usize, usize)>) -> usize {
        (0..map.len()).flat_map(|i| (0..map[0].len()).map(move |j| (i, j)))
            .filter(
                |&(i, j)| !pipe.contains(&(i, j))
                    && map[i][..j].iter().enumerate()
                        .filter(|&(j2, c)|
                                pipe.contains(&(i, j2))
                                && vec!['|', 'F', '7'].contains(c))
                        .count() % 2 == 1)
            .count()
    }

    pub fn solve_a(input: &str) -> usize {
        let mut map = parse(input);
        mark_pipes(&mut map).0
    }

    pub fn solve_b(input: &str) -> usize {
        let mut map = parse(input);
        let (_, pipe) = mark_pipes(&mut map);
        count_inside(&map, &pipe)
    }

    #[cfg(test)]
    mod test_day10 {
        use super::*;

        static INPUT1: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        static INPUT2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        static INPUT3: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT1), 8);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT2), 4);
            assert_eq!(solve_b(INPUT3), 10);
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

    println!("Solution A-part: {}", day10::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day10::solve_b(&buffer.trim()));
}
