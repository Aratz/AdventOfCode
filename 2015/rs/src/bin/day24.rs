mod day24 {
    use std::cmp::max;
    use std::collections::HashMap;

    type State = (usize, i64, i64, i64);
    type Grouping = (usize, i64);

    fn parse(input: &str) -> Vec<i64> {
        input.lines()
            .map(|l| l.parse().unwrap())
            .collect()
    }

    fn solve_dp(
        state: State,
        weights: &[i64],
        groupings: &mut HashMap<State, Option<Grouping>>) -> Option<Grouping> {
        if let Some(grp) = groupings.get(&state) {
            *grp
        }
        else {
            let (i_p, l1, l2, l3) = state;
            let mut grouping = Vec::new();

            if i_p == weights.len() - 1 {
                if l1 == weights[i_p] && l2 == 0 && l3 == 0 {
                    grouping.push((1, weights[i_p]));
                }
                if l1 == 0 && l2 == weights[i_p] && l3 == 0 {
                    grouping.push((0, 0));
                }
                if l1 == 0 && l2 == 0 && l3 == weights[i_p] {
                    grouping.push((0, 0));
                }
            }
            else {
                if l1 >= weights[i_p] {
                    if let Some((n_p, quant)) = solve_dp(
                        (i_p + 1, l1 - weights[i_p], l2, l3),
                        weights, groupings) {
                        grouping.push((n_p + 1, weights[i_p] * max(1, quant)));
                    }
                }
                if l2 >= weights[i_p] {
                    if let Some(grp) = solve_dp(
                        (i_p + 1, l1, l2 - weights[i_p], l3),
                        weights, groupings) {
                        grouping.push(grp);
                    }
                }
                if l3 >= weights[i_p] {
                    if let Some(grp) = solve_dp(
                        (i_p + 1, l1, l2, l3 - weights[i_p]),
                        weights, groupings) {
                        grouping.push(grp);
                    }
                }
            }

            if let Some(grp) = grouping.into_iter().min() {
                groupings.insert(state, Some(grp));
                Some(grp)
            }
            else {
                groupings.insert(state, None);
                None
            }
        }
    }

    pub fn solve_a(input: &str) -> i64 {
        let packets = parse(input);
        let max_weight = packets.iter().sum::<i64>()/3;

        let mut groupings = HashMap::new();
        let init = (0, max_weight, max_weight, max_weight);

        solve_dp(
            init,
            &packets,
            &mut groupings);


        groupings[&init].as_ref().unwrap().1
    }

    pub fn solve_b(input: &str) -> i64 {
        let packets = parse(input);
        let max_weight = packets.iter().sum::<i64>()/4;

        let mut groupings = HashMap::new();
        groupings.insert((packets[0], 0, 0), (1, packets[0]));
        groupings.insert((0, packets[0], 0), (0, 0));
        groupings.insert((0, 0, packets[0]), (0, 0));
        groupings.insert((0, 0, 0), (0, 0));

        for (i, packet) in packets.iter().enumerate().skip(1) {
            let unloaded: i64 = packets[..i].iter().sum();

            let mut new_groupings = HashMap::new();

            for ((l1, l2, l3), (n_p, quant)) in groupings.into_iter() {
                let mut v = vec![];

                if l1 + packet <= max_weight {
                    v.push(((l1 + packet, l2, l3), (n_p + 1, max(1, quant)*packet)));
                }
                if l2 + packet <= max_weight {
                    v.push(((l1, l2 + packet, l3), (n_p, quant)));
                }
                if l3 + packet <= max_weight {
                    v.push(((l1, l2, l3 + packet), (n_p, quant)));
                }
                if unloaded - l1 - l2 - l3 + packet <= max_weight {
                    v.push(((l1, l2, l3), (n_p, quant)));
                }

                for (state, val) in v {
                    if let Some(old_val) = new_groupings.get(&state) {
                        if val < *old_val {
                            new_groupings.insert(state, val);
                        }
                    }
                    else {
                        new_groupings.insert(state, val);
                    }
                }
            }

            groupings = new_groupings;
        }
        let init = (max_weight, max_weight, max_weight);
        groupings[&init].1
    }

    #[cfg(test)]
    mod test_day24 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let input = "1
2
3
4
5
7
8
9
10
11";
            assert_eq!(solve_a(&input), 99);

        }

        #[test]
        fn test_solve_b() {
            let input = "1
2
3
4
5
7
8
9
10
11";
            assert_eq!(solve_b(&input), 44);
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

    println!("Solution A-part: {}", day24::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day24::solve_b(&buffer.trim()));
}
