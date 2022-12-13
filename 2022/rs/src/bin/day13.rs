mod day13 {
    use std::cmp::Ordering;

    #[derive(PartialEq, Debug, Clone)]
    enum Packet {
        Number(u32),
        List(Vec<Packet>),
    }

    fn parse_next(input: &[char], mut pos: usize) -> (Packet, usize) {
        match input[pos] {
            '[' => {
                let mut content = vec![];
                pos += 1;
                while pos < input.len() && input[pos] != ']' {
                    let (pack, new_pos) = parse_next(input, pos);
                    content.push(pack);
                    pos = new_pos;
                    if pos < input.len() && input[pos] == ',' {
                        pos += 1;
                    }
                }
                (Packet::List(content), pos + 1)
            }
            x if '0' <= x && x <= '9' => {
                if pos + 1 < input.len() && input[pos + 1] == '0' {
                    (Packet::Number(10), pos + 2)
                }
                else {
                    (Packet::Number(x.to_digit(10).unwrap()), pos + 1)
                }
            }
            _ => unreachable!(),
        }
    }

    impl Packet {
        fn new(input: &str) -> Self {
            parse_next(&input.chars().collect::<Vec<char>>(), 0).0
        }
    }

    impl PartialOrd for Packet {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match (self, other) {
                (Packet::Number(n_self), Packet::Number(n_other)) => {
                    if n_self == n_other { None }
                    else { Some(n_self.cmp(n_other)) }
                },
                (Packet::List(l_self), Packet::List(l_other)) => {
                    match l_self.iter().zip(l_other.iter())
                        .filter_map(|pair| pair.0.partial_cmp(pair.1))
                        .next() {
                        Some(res) => { Some(res) },
                        None => {
                            if l_self.len() == l_other.len() { None }
                            else { Some(l_self.len().cmp(&l_other.len())) }
                        },
                    }
                },
                (Packet::List(_), Packet::Number(_)) => {
                    self.partial_cmp(&Packet::List(vec![other.clone()]))
                },
                (Packet::Number(_), Packet::List(_)) => {
                    Packet::List(vec![self.clone()]).partial_cmp(other)
                },
            }
        }
    }

    pub fn solve_a(input: &str) -> usize {
        input.split("\n\n")
            .enumerate()
            .map(|(i, raw_pairs)| {
                let mut pair_iter = raw_pairs.split('\n');
                let first_pair = Packet::new(pair_iter.next().unwrap());
                let second_pair = Packet::new(pair_iter.next().unwrap());
                (i + 1, (first_pair, second_pair))
            })
            .filter(|(_i, pairs)| match pairs.0.partial_cmp(&pairs.1) {
                Some(Ordering::Less) => true,
                _ => false,
            })
            .map(|(i, _)| i)
            .sum()
    }

    pub fn solve_b(input: &str) -> usize {
        let mut packets = input.replace("\n\n", "\n").lines()
            .map(|l| Packet::new(l))
            .collect::<Vec<_>>();
        packets.push(Packet::new("[[2]]"));
        packets.push(Packet::new("[[6]]"));

        packets.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        (
            packets.iter()
                .enumerate()
                .find(|(_, packet)| packet == &&Packet::new("[[2]]")).unwrap().0 + 1)
        * (
            packets.iter()
                .enumerate()
                .find(|(_, packet)| packet == &&Packet::new("[[6]]")).unwrap().0 + 1)
    }

    #[cfg(test)]
    mod test_day13 {
        use super::*;

        static INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";


        #[test]
        fn test_parse() {
            assert_eq!(
                Packet::new("[[4,4],4,4]"),
                Packet::List(vec![
                    Packet::List(vec![Packet::Number(4), Packet::Number(4)]),
                    Packet::Number(4),
                    Packet::Number(4),
                ]));
            assert_eq!(
                Packet::new("[[]]"),
                Packet::List(vec![Packet::List(vec![])]));
        }

        #[test]
        fn test_ord() {
            assert_eq!(
                Packet::new("[]").partial_cmp(&Packet::new("[3]")),
                Some(Ordering::Less));
        }

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(&INPUT), 13);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(&INPUT), 140);
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

    println!("Solution A-part: {}", day13::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day13::solve_b(&buffer.trim()));
}
