mod day04 {
    fn parse(input: &str) -> Vec<((i32, i32), (i32, i32))> {
        input.lines()
            .map(|l| {
                let mut ranges = l.split(',')
                    .map(|range| {
                         let mut borders = range.split('-');
                         (
                             borders.next().unwrap().parse().unwrap(),
                             borders.next().unwrap().parse().unwrap()
                         )
                    });
                (ranges.next().unwrap(), ranges.next().unwrap())
            })
            .collect()
    }

    fn is_inside(a: &(i32, i32), b: &(i32, i32)) -> bool {
        b.0 <= a.0 && a.0 <= b.1 && a.1 <= b.1
    }


    fn do_overlap(a: &(i32, i32), b: &(i32, i32)) -> bool {
        !(a.1 < b.0 || b.1 < a.0)
    }

    pub fn solve_a(input: &str) -> usize {
        parse(input).into_iter()
        .filter(|(a, b)| is_inside(a, b) || is_inside(b, a))
        .count()
    }

    pub fn solve_b(input: &str) -> usize {
        parse(input).into_iter()
        .filter(|(a, b)| do_overlap(a, b))
        .count()
    }

    #[cfg(test)]
    mod test_day04 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
            assert_eq!(solve_a(&input), 2);
        }

        #[test]
        fn test_solve_b() {
            let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
            assert_eq!(solve_b(&input), 4);
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

    println!("Solution A-part: {}", day04::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day04::solve_b(&buffer.trim()));
}
