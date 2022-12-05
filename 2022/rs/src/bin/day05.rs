mod day05 {
    fn parse(input: & str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
        let mut stacksinsts = input.split("\n\n");
        let raw_stacks = stacksinsts.next().unwrap();
        let raw_insts = stacksinsts.next().unwrap();

        let mut stacks = vec![vec![]; 9];

        for l in raw_stacks.lines() {
            if l.contains("1") { break; }
            for (i, c) in l.chars().skip(1).step_by(4).enumerate() {
                if c != ' ' {
                    stacks[i].push(c)
                }
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }

        let insts = raw_insts.lines()
            .map(|l| {
                let words = l.split(' ').collect::<Vec<_>>();
                (
                    words[1].parse().unwrap(),
                    words[3].parse().unwrap(),
                    words[5].parse().unwrap()
                )
            })
            .collect::<Vec<(usize, usize, usize)>>();

        (stacks, insts)
    }

    pub fn solve_a(input: &str) -> String {
        let (mut stacks, insts) = parse(input);

        for (n, from, to) in insts.into_iter() {
            for _ in 0..n {
                let crt = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(crt);
            }
        }

        stacks.into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s[s.len() - 1]).collect()
    }

    pub fn solve_b(input: &str) -> String {
        let (mut stacks, insts) = parse(input);

        for (n, from, to) in insts.into_iter() {
            let stack_size = stacks[from - 1].len();
            let mut hook: Vec<char> = stacks[from - 1].drain(stack_size - n..).collect();
            stacks[to - 1].append(&mut hook);
        }

        stacks.into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s[s.len() - 1]).collect()
    }

    #[cfg(test)]
    mod test_day05 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

            assert_eq!(solve_a(&input), "CMZ");
        }

        #[test]
        fn test_solve_b() {
            let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

            assert_eq!(solve_b(&input), "MCD");
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

    println!("Solution A-part: {}", day05::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day05::solve_b(&buffer.trim()));
}
