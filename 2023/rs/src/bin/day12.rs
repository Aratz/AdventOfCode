mod day12 {
    use std::cmp::min;
    use std::collections::HashMap;

    type Cache = HashMap<(String, Vec<usize>), usize>;

    fn parse(input: &str) -> Vec<(String, Vec<usize>)> {
        input.lines()
            .map(
                |l| {
                    let rec_check = l.split(' ').collect::<Vec<_>>();
                    let record = rec_check[0].to_string();
                    let check = rec_check[1]
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect();
                    (record, check)
                })
            .collect()
    }

    fn gen_check(record: &str) -> Vec<usize> {
        record.split('.')
            .map(|s| s.len())
            .filter(|&s_len| s_len > 0)
            .collect()
    }


    fn n_arr(record: &str, check: &[usize], cache: &mut Cache) -> usize {
        if let Some(&res) = cache.get(&(record.to_string(), check.to_vec())) {
            return res;
        }

        let res = {
            if check.is_empty() {
                return if record.contains('#') { 0 }
                       else { 1 }
            }

            if record.is_empty() {
                return if check.is_empty() { 1 }
                       else { 0 }
            }

            let mut total = 0;
            let spring = record.chars().next().unwrap();

            if spring == '.' || spring == '?' {
                total += n_arr(&record[1..], check, cache);
            }

            if spring == '#' || spring == '?' {
                let c0 = check[0];
                if record.len() >= c0
                    && !record[0..c0].contains('.')
                    && (
                        record.len() == c0
                        || ".?".contains(record.chars().nth(c0).unwrap())
                    )
                {
                    total += n_arr(&record[min(record.len(),c0+1)..], &check[1..], cache);
                }
            }
            total
        };
        cache.insert((record.to_string(), check.to_vec()), res);

        res
    }

    pub fn solve_a(input: &str) -> usize {
        let mut cache = Cache::new();
        parse(input).into_iter()
            .map(|(rec, check)| n_arr(&rec, &check, &mut cache))
            .sum()
    }

    pub fn solve_b(input: &str) -> usize {
        let rep = 5;
        let mut cache = Cache::new();
        parse(input).into_iter()
            .map(|(rec, check)| (
                    vec![rec].into_iter().cycle().take(rep)
                        .collect::<Vec<String>>().join("?"),
                    check.repeat(rep)))
            .map(|(rec, check)| n_arr(&rec, &check, &mut cache))
            .sum()
    }

    #[cfg(test)]
    mod test_day12 {
        use super::*;

        #[test]
        fn test_n_arr() {
            let mut cache = Cache::new();
            assert_eq!(n_arr("....###", &[3], &mut cache), 1);
            assert_eq!(n_arr("....###.", &[3], &mut cache), 1);
            assert_eq!(n_arr(".#..###", &[3], &mut cache), 0);
            assert_eq!(n_arr("???.###", &[1, 1, 3], &mut cache), 1);
            assert_eq!(n_arr(".??..??...?##.", &[1, 1, 3], &mut cache), 4);
            assert_eq!(n_arr("?#?#?#?#?#?#?#?", &[1, 3, 1, 6], &mut cache), 1);
            assert_eq!(n_arr("????.#...#...", &[4, 1, 1], &mut cache), 1);
            assert_eq!(n_arr("????.######..#####.", &[1, 6, 5], &mut cache), 4);
            assert_eq!(n_arr("?###????????", &[3, 2, 1], &mut cache), 10);
            assert_eq!(n_arr("???", &[2], &mut cache), 2);
        }

        static INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 21);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 525_152);
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

    println!("Solution A-part: {}", day12::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day12::solve_b(&buffer.trim()));
}
