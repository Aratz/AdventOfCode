mod day09 {
    use std::collections::HashSet;
    pub fn solve_ab(input: &str, rope_len: usize) -> usize {
        let mut tail_pos = HashSet::new();

        let mut rope: Vec<(i32, i32)> = vec![(0, 0); rope_len];

        for line in input.lines() {
            let words: Vec<_> = line.split(' ').collect();
            let len: i32 = words[1].parse().unwrap();

            for _ in 0..len {
                match words[0] {
                    "U" => {
                        rope[0] = (rope[0].0, rope[0].1 + 1);
                    },
                    "R" => {
                        rope[0] = (rope[0].0 + 1, rope[0].1);
                    },
                    "D" => {
                        rope[0] = (rope[0].0, rope[0].1 - 1);
                    },
                    "L" => {
                        rope[0] = (rope[0].0 - 1, rope[0].1);
                    },
                    _ => unreachable!(),
                }

                for i_knot in 1..rope_len {
                    if rope[i_knot].0.abs_diff(rope[i_knot-1].0) > 1
                            || rope[i_knot].1.abs_diff(rope[i_knot - 1].1) > 1 {
                        let diff = (
                            rope[i_knot - 1].0 - rope[i_knot].0,
                            rope[i_knot - 1].1 - rope[i_knot].1,
                            );

                        let dir = (
                            if diff.0 == 0 { 0 } else { diff.0/diff.0.abs() },
                            if diff.1 == 0 { 0 } else { diff.1/diff.1.abs() },
                            );

                        rope[i_knot] = (rope[i_knot].0 + dir.0, rope[i_knot].1 + dir.1);
                    }
                }

                tail_pos.insert(rope[rope_len - 1]);
            }
        }

        tail_pos.len()
    }

    #[cfg(test)]
    mod test_day09 {
        use super::*;


        #[test]
        fn test_solve_ab() {
            let input: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
            assert_eq!(solve_ab(&input, 2), 13);
            assert_eq!(solve_ab(&input, 10), 1);

            let input2 = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
            assert_eq!(solve_ab(&input2, 10), 36);
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

    println!("Solution A-part: {}", day09::solve_ab(&buffer.trim(), 2));
    println!("Solution B-part: {}", day09::solve_ab(&buffer.trim(), 10));
}
