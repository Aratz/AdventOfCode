mod day18 {
    use std::ops;

    #[derive(PartialEq, Debug, Clone)]
    enum Number {
        Pair((Box<Number>, Box<Number>)),
        Regular(u32),
    }

    impl Number {
        fn new(s: &mut dyn Iterator<Item=char>) -> Self {
            match s.next() {
                Some(c) if c.is_numeric() => { Number::Regular(c.to_digit(10).unwrap()) },
                Some('[') => {
                    let left = Number::new(s);

                    if s.next() != Some(',') { unreachable!(); }

                    let right = Number::new(s);

                    if s.next() != Some(']') { unreachable!(); }

                    Number::Pair((Box::new(left), Box::new(right)))
                },
                _ => unreachable!(),
            }
        }

        fn explode(&mut self, depth: usize) -> Option<(Option<u32>, Option<u32>)> {
            match self {
                Number::Pair((left, right)) => {
                    if depth == 4 {
                        let left = match **left {
                            Number::Pair(_) => unreachable!(),
                            Number::Regular(n) => n,
                        };

                        let right = match **right {
                            Number::Pair(_) => unreachable!(),
                            Number::Regular(n) => n,
                        };

                        *self = Number::Regular(0);

                        Some((Some(left), Some(right)))
                    }
                    else {
                        if let Some((l, r)) = left.explode(depth + 1) {
                            if let Some(v) = r {
                                right.left_add(v);
                            }
                            return Some((l, None));
                        }

                        if let Some((l, r)) = right.explode(depth + 1) {
                            if let Some(v) = l {
                                left.right_add(v);
                            }
                            return Some((None, r));
                        }

                        None
                    }
                },
                Number::Regular(_) => { None },
            }
        }

        fn left_add(&mut self, v: u32) {
            match self {
                Number::Regular(n) => { *n += v; },
                Number::Pair((left, _right)) => left.left_add(v),
            }
        }

        fn right_add(&mut self, v: u32) {
            match self {
                Number::Regular(n) => { *n += v; },
                Number::Pair((_left, right)) => right.right_add(v),
            }
        }

        fn split(&mut self) -> bool {
            match self {
                Number::Regular(n) => {
                    if *n >= 10 {
                        *self = Number::Pair((
                                Box::new(Number::Regular(*n / 2)),
                                Box::new(Number::Regular(*n - *n/2)),
                            ));
                        true
                    }
                    else {
                        false
                    }
                },
                Number::Pair((left, right)) => {
                    if !left.split() {
                        right.split()
                    }
                    else {
                        true
                    }
                },
            }
        }

        fn reduce(&mut self) {
            loop {
                if let Some(_) = self.explode(0) { continue; }

                if !self.split() { break; }
            }
        }

        fn magnitude(&self) -> u32 {
            match self {
                Number::Regular(n) => { *n },
                Number::Pair((left, right)) => { 3*left.magnitude() + 2*right.magnitude()},
            }
        }
    }

    impl ops::Add<&Number> for Number {
    type Output = Number;

        fn add(self, other: &Number) -> Number {
            let mut res = Number::Pair((
                Box::new(self.clone()),
                Box::new(other.clone())
            ));
            res.reduce();

            res
        }
    }

    pub fn solve_a(input: &[String]) -> u32 {
        let mut numbers: Vec<Number> = input.iter()
            .map(|s| Number::new(&mut s.chars())).collect();

        let init = numbers.remove(0);
        numbers.iter()
            .fold(init, |acc, x| acc + x)
            .magnitude()
    }

    pub fn solve_b(input: &[String]) -> u32 {
        let numbers: Vec<Number> = input.iter()
            .map(|s| Number::new(&mut s.chars())).collect();

        numbers.iter().flat_map(|a| numbers.iter().map(move |b| (a.clone(), b.clone())))
            .filter(|(a, b)| a != b)
            .map(|(a, b)| a + &b)
            .map(|n| n.magnitude())
            .max().unwrap()
    }


    #[cfg(test)]
    mod test_day18 {
        use super::*;

        #[test]
        fn test_new() {
            let s = "[1,2]";
            assert_eq!(
                Number::new(&mut s.chars()),
                Number::Pair((
                            Box::new(Number::Regular(1)),
                            Box::new(Number::Regular(2)))));

            let s = "[[1,2],3]";
            assert_eq!(
                Number::new(&mut s.chars()),
                Number::Pair((
                            Box::new(Number::Pair((
                                        Box::new(Number::Regular(1)),
                                        Box::new(Number::Regular(2))))),
                            Box::new(Number::Regular(3)))));
        }

        #[test]
        fn test_explode() {
            let mut pair = Number::new(&mut "[[[[[9,8],1],2],3],4]".chars());
            pair.explode(0);
            assert_eq!(pair, Number::new(&mut "[[[[0,9],2],3],4]".chars()));

            let mut pair = Number::new(&mut "[7,[6,[5,[4,[3,2]]]]]".chars());
            pair.explode(0);
            assert_eq!(pair, Number::new(&mut "[7,[6,[5,[7,0]]]]".chars()));

            let mut pair = Number::new(&mut "[[6,[5,[4,[3,2]]]],1]".chars());
            pair.explode(0);
            assert_eq!(pair, Number::new(&mut "[[6,[5,[7,0]]],3]".chars()));

            let mut pair = Number::new(&mut "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".chars());
            pair.explode(0);
            assert_eq!(pair, Number::new(&mut "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars()));

            let mut pair = Number::new(&mut "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars());
            pair.explode(0);
            assert_eq!(pair, Number::new(&mut "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".chars()));

            let mut pair = Number::new(&mut "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]".chars());
            pair.explode(0);
            assert_eq!(pair, Number::new(&mut "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]".chars()));
            pair.explode(0);
            assert_eq!(pair, Number::new(&mut "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]".chars()));
        }

        #[test]
        fn test_split() {
            let mut pair = Number::Pair((
                    Box::new(Number::Regular(1)),
                    Box::new(Number::Regular(11)),
                             ));
            assert!(pair.split());
            assert_eq!(pair, Number::new(&mut "[1,[5,6]]".chars()));
        }

        #[test]
        fn test_reduce() {
            let mut pair = Number::new(&mut "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".chars());
            pair.reduce();
            assert_eq!(pair, Number::new(&mut "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".chars()));

            pair.reduce();
            assert_eq!(pair, Number::new(&mut "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".chars()));

            let mut pair = Number::new(&mut "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]".chars());
            pair.reduce();
            assert_eq!(pair, Number::new(&mut "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".chars()));
        }

        #[test]
        fn test_magnitude() {
            let pair = Number::new(&mut "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]".chars());
            assert_eq!(pair.magnitude(), 3993);
        }

        #[test]
        fn test_add() {
            let pair1 = Number::new(&mut "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".chars());
            let pair2 = Number::new(&mut "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]".chars());

            assert_eq!(pair1 + &pair2, Number::new(&mut "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]".chars()));
        }

        #[test]
        fn test_solve_a() {
            let input: Vec<String> = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".lines().map(|s| s.to_string()).collect();

            assert_eq!(solve_a(&input), 4140);
        }

        #[test]
        fn test_solve_b() {
            let input: Vec<String> = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".lines().map(|s| s.to_string()).collect();

            assert_eq!(solve_b(&input), 3993);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let raw_numbers: Vec<String> = stdin.lock().lines()
        .map(|l| l.unwrap()).collect();

    println!("Solution A-part: {}", day18::solve_a(&raw_numbers));
    println!("Solution B-part: {}", day18::solve_b(&raw_numbers));
}
