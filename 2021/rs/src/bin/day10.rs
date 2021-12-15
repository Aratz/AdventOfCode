mod day10 {
    use std::collections::{HashMap, VecDeque};

    fn is_corrupted(line: &str) -> Option<char> {
        let mut stack = VecDeque::new();

        let matching_brckt: HashMap<_, _> = vec![
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
            ('<', '>')
        ].into_iter().collect();

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push_back(c);
                },
                ')' | ']' | '}' | '>' => {
                    if stack.is_empty() || matching_brckt[&stack.pop_back().unwrap()] != c {
                        return Some(c)
                    }
                },
                _ => panic!("Illegal character!"),
            }
        }

        None
    }

    fn autocomplete(line: &str) -> Vec<char> {
        assert!(is_corrupted(line).is_none());

        let mut stack = VecDeque::new();

        let matching_brckt: HashMap<_, _> = vec![
            ('(', ')'),
            ('[', ']'),
            ('{', '}'),
            ('<', '>')
        ].into_iter().collect();

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push_back(c);
                },
                ')' | ']' | '}' | '>' => {
                    stack.pop_back();
                },
                _ => panic!("Illegal character!"),
            }
        }

        stack.make_contiguous();
        let mut seq = stack.as_slices().0.to_vec();
        seq.reverse();

        seq.iter().map(|c| matching_brckt[&c]).collect()
    }

    pub fn solve_a(lines: &[String]) -> i64 {
        let base_scores: HashMap<char, i64> = vec![
                (')', 3),
                (']', 57),
                ('}', 1197),
                ('>', 25137)
            ].into_iter().collect();

        lines.iter()
            .filter_map(|l| is_corrupted(&l))
            .map(|c| base_scores[&c])
            .sum()
    }

    pub fn solve_b(lines: &[String]) -> i64 {
        let base_scores: HashMap<char, i64> = vec![
                (')', 1),
                (']', 2),
                ('}', 3),
                ('>', 4)
            ].into_iter().collect();

        let mut scores: Vec<i64> = lines.iter()
            .filter(|l| is_corrupted(l).is_none())
            .map(|l| autocomplete(&l))
            .map(|v| v.iter()
                 .map(|c| base_scores[&c])
                 .fold(0, |acc, x| acc*5 + x))
            .collect();

        scores.sort_unstable();

        scores[scores.len()/2]
    }


    #[cfg(test)]
    mod test_day10 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let lines = vec![String::from("[({(<(())[]>[[{[]{<()<>>")];
            assert_eq!(solve_a(&lines), 0);

            let lines = vec![
                String::from("[({(<(())[]>[[{[]{<()<>>"),
                String::from("[(()[<>])]({[<{<<[]>>("),
                String::from("{([(<{}[<>[]}>{[]{[(<()>"),
                String::from("(((({<>}<{<{<>}{[]{[]{}"),
                String::from("[[<[([]))<([[{}[[()]]]"),
                String::from("[{[{({}]{}}([{[{{{}}([]"),
                String::from("{<[[]]>}<{[{[{[]{()[[[]"),
                String::from("[<(<(<(<{}))><([]([]()"),
                String::from("<{([([[(<>()){}]>(<<{{"),
                String::from("<{([{{}}[<[[[<>{}]]]>[]]")
            ];

            assert_eq!(solve_a(&lines), 26397);
        }

        #[test]
        fn test_solve_b() {
            let lines = vec![
                String::from("[({(<(())[]>[[{[]{<()<>>"),
                String::from("[(()[<>])]({[<{<<[]>>("),
                String::from("{([(<{}[<>[]}>{[]{[(<()>"),
                String::from("(((({<>}<{<{<>}{[]{[]{}"),
                String::from("[[<[([]))<([[{}[[()]]]"),
                String::from("[{[{({}]{}}([{[{{{}}([]"),
                String::from("{<[[]]>}<{[{[{[]{()[[[]"),
                String::from("[<(<(<(<{}))><([]([]()"),
                String::from("<{([([[(<>()){}]>(<<{{"),
                String::from("<{([{{}}[<[[[<>{}]]]>[]]")
            ];

            assert_eq!(solve_b(&lines), 288957);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines: Vec<String> = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect();

    println!("Solution A-part: {}", day10::solve_a(&lines));
    println!("Solution B-part: {}", day10::solve_b(&lines));
}
