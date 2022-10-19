extern crate regex;
extern crate itertools;

mod day13 {
    use regex::Regex;
    use itertools::Itertools;
    use std::collections::HashMap;

    type CostMatrix = HashMap<String, HashMap<String, i32>>;

    fn parse(input: &str, include_self: bool) -> CostMatrix {
        let mut mat = CostMatrix::new();

        let re_dist = Regex::new(r"(?P<A>\w+) would (?P<sign>gain|lose) (?P<cost>\d+) happiness units by sitting next to (?P<B>\w+)").unwrap();

        for capt in re_dist.captures_iter(input) {
            let cost = match &capt["sign"] {
                "gain" => 1,
                "lose" => -1,
                _ => unreachable!()} * capt["cost"].parse::<i32>().unwrap();
            mat.entry(capt["A"].to_string())
                .or_default()
                .insert(capt["B"].to_string(), cost);
        }

        if include_self {
            let guests = mat.keys().cloned().collect::<Vec<_>>();
            for guest in guests {
                mat.entry(guest.to_string())
                    .or_default()
                    .insert("Adrien".to_string(), 0);
                mat.entry("Adrien".to_string())
                    .or_default()
                    .insert(guest.to_string(), 0);
            }
        }

        mat
    }

    fn sum_perms<'a>(costs: &'a CostMatrix) -> impl Iterator<Item=i32> + 'a {
        costs.keys()
            .permutations(costs.len())
            .map(move |v| v.iter()
                 .zip(v.iter().cycle().skip(1))
                 .map(|(a, b)| costs[a.as_str()][b.as_str()] + costs[b.as_str()][a.as_str()])
                 .sum::<i32>())
    }

    pub fn solve_a(input: &str) -> i32 {
        let dists = parse(input, false);
        sum_perms(&dists).max().unwrap()
    }

    pub fn solve_b(input: &str) -> i32 {
        let dists = parse(input, true);
        sum_perms(&dists).max().unwrap()
    }

    #[cfg(test)]
    mod test_day13 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

            assert_eq!(solve_a(&input), 330);
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
