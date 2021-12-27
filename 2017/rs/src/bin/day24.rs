mod day24 {
    use std::collections::HashMap;
    use std::cmp::max;

    type Comp = (i32, i32);

    fn has_common_pin(c1: Comp, c2: Comp) -> bool {
        c1.0 == c2.0 || c1.0 == c2.1
     || c1.1 == c2.0 || c1.1 == c2.1
    }

    fn parse(input: &[String]) -> HashMap<Comp, Vec<Comp>> {
        let comps: Vec<Comp> = input.iter()
            .map(|s| {
                let pins: Vec<i32> = s.split('/').map(|pins| pins.parse().unwrap()).collect();
                (pins[0], pins[1])
            }).collect();

        let mut cnx: HashMap<Comp, Vec<Comp>> = HashMap::new();

        for c1 in &comps {
            for c2 in &comps {
                if has_common_pin(*c1, *c2) {
                    cnx.entry(*c1).or_default().push(*c2);
                }
            }
        }

        cnx
    }

    fn get_strength(path: &[Comp], ab: bool) -> (usize, i32) {
        let strength = path.iter()
            .map(|c| c.0 + c.1)
            .sum::<i32>();

        if !ab {
            (0, strength)
        }
        else {
            (path.len(), strength)
        }
    }

    pub fn solve(input: &[String], ab: bool) -> i32 {
        let cnx = parse(&input);

        let mut stack: Vec<(Vec<Comp>, i32)> = cnx.keys()
            .filter(|&&k| has_common_pin(k, (0, 0)))
            .map(|&k| (vec![k], if k.0 != 0 { k.0 } else { k.1 }))
            .collect();

        let mut max_strength = (0, 0);

        while let Some((path, last)) = stack.pop() {
            max_strength = max(max_strength, get_strength(&path, ab));

            for c in cnx[&path[path.len() - 1]].iter()
                .filter(|&&c| has_common_pin(c, (last, last)))
                .filter(|c1| path.iter().all(|c2| *c1 != c2)) {
                let mut new_path = path.clone();
                new_path.push(*c);
                let new_last = if c.0 != last { c.0 } else { c.1 };

                stack.push((new_path, new_last));
            }
        }

        max_strength.1
    }

    #[cfg(test)]
    mod test_day24 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input: Vec<String> = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10".lines().map(|l| l.to_string()).collect();

            assert_eq!(solve(&input, false), 31);
        }

        #[test]
        fn test_solve_b() {
            let input: Vec<String> = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10".lines().map(|l| l.to_string()).collect();

            assert_eq!(solve(&input, true), 19);
        }
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let input: Vec<String> = stdin.lock().lines()
        .map(|l| l.unwrap()).collect();

    println!("Solution A-part: {}", day24::solve(&input, false));
    println!("Solution B-part: {}", day24::solve(&input, true));
}
