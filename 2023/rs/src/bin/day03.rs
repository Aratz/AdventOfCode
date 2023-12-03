extern crate regex;

mod day03 {
    use regex::Regex;
    use std::cmp::min;
    use std::collections::HashMap;

    pub fn solve_a(input: &str) -> i32 {
        let re_num = Regex::new(r"[0-9]+").unwrap();

        let carta = input.lines().map(
            |l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (h, w) = (carta.len(), carta[0].len());

        input.lines().enumerate().flat_map(
            |(i, l)| re_num.find_iter(l)
                .map(move |m_number| (
                        m_number,
                        (if i >= 1 { i - 1 } else { 0 }, min(h, i + 2))
                    )
                )
                .filter(
                    |(m_number, (min_h, max_h))| {
                    let (min_w, max_w) = (
                        if m_number.start() >= 1 { m_number.start() - 1} else { 0 },
                        min(w, m_number.end() + 1));

                    carta[*min_h..*max_h].iter()
                        .flat_map(
                            |row| row[min_w..max_w].iter())
                        .any(
                            |c| match c {
                                '0'..='9' | '.' => false,
                                _ => true,
                            }
                        )
                    })
            )
            .map(|(m_number, _)| m_number.as_str().parse::<i32>().unwrap())
            .sum()
    }

    pub fn solve_b(input: &str) -> i32 {
        let re_num = Regex::new(r"[0-9]+").unwrap();

        let carta = input.lines().map(
            |l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (h, w) = (carta.len(), carta[0].len());

        let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
        for (i, l) in input.lines().enumerate() {
            for m_number in re_num.find_iter(l) {
                let j = m_number.start();
                let (min_h, max_h) = (
                    if i >= 1 { i - 1 } else { 0 },
                    min(h, i + 2)
                );
                let (min_w, max_w) = (
                    if j >= 1 { j - 1} else { 0 },
                    min(w, m_number.end() + 1)
                );

                for (g_i, g_j) in carta[min_h..max_h].iter().enumerate()
                        .flat_map(
                            |(g_i, row)| row[min_w..max_w].iter().enumerate()
                            .filter(|(g_j, &c)| c == '*')
                            .map(move |(g_j, _)| (g_i, g_j))
                        ) {
                    gears.entry((min_h + g_i, min_w + g_j)).or_default().push(
                        m_number.as_str().parse::<i32>().unwrap());

                }
            }
        }

        gears.values()
            .filter(|nums| nums.len() == 2)
            .map(|nums| nums.iter().product::<i32>())
            .sum()
    }

    #[cfg(test)]
    mod test_day03 {
        use super::*;

        static INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 4361);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 467835);
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

    println!("Solution A-part: {}", day03::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day03::solve_b(&buffer.trim()));
}
