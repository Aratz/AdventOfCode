extern crate regex;
extern crate itertools;

mod day09 {
    use regex::Regex;
    use itertools::Itertools;
    use std::collections::HashMap;

    type DistMatrix = HashMap<String, HashMap<String, i32>>;

    fn parse(input: &str) -> DistMatrix {
        let mut mat = DistMatrix::new();

        let re_dist = Regex::new(r"(?P<A>\w+) to (?P<B>\w+) = (?P<dist>\d+)").unwrap();

        for capt in re_dist.captures_iter(input) {
            mat.entry(capt["A"].to_string())
                .or_default()
                .insert(capt["B"].to_string(), capt["dist"].parse().unwrap());
            mat.entry(capt["B"].to_string())
                .or_default()
                .insert(capt["A"].to_string(), capt["dist"].parse().unwrap());
        }

        mat
    }

    fn sum_perms<'a>(dists: &'a DistMatrix) -> impl Iterator<Item=i32> + 'a {
        dists.keys()
            .permutations(dists.len())
            .map(move |v| v.iter()
                 .zip(v.iter().skip(1))
                 .map(|(a, b)| dists[a.as_str()][b.as_str()])
                 .sum::<i32>())
    }

    pub fn solve_a(input: &str) -> i32 {
        let dists = parse(input);
        sum_perms(&dists).min().unwrap()
    }

    pub fn solve_b(input: &str) -> i32 {
        let dists = parse(input);
        sum_perms(&dists).max().unwrap()
    }

    #[cfg(test)]
    mod test_day09 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

            assert_eq!(solve_a(input), 605);
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

    println!("Solution A-part: {}", day09::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day09::solve_b(&buffer.trim()));
}
