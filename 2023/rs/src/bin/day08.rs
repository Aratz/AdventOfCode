extern crate regex;

mod day08 {
    use regex::Regex;
    use std::collections::HashMap;

    fn parse(input: &str) -> (String, HashMap<String, HashMap<char, String>>) {
        let re_network = Regex::new(r"(?P<node>\w+) = \((?P<L>\w+), (?P<R>\w+)\)").unwrap();

        let route_network = input.split("\n\n").collect::<Vec<_>>();

        (
            route_network[0].to_string(),
            re_network.captures_iter(route_network[1])
                .map(|capt| (
                    capt["node"].to_string(),
                    vec![
                        ('L', capt["L"].to_string()),
                        ('R', capt["R"].to_string()),
                    ].into_iter().collect()
                )).collect()
        )
    }

    pub fn solve_a(input: &str) -> usize {
        let (route, network) = parse(input);

        route.chars().cycle()
            .scan("AAA".to_string(),
                |node, dir| {
                    let new_node = &network[node][&dir];
                    *node = new_node.clone();
                    Some(new_node)
                })
            .take_while(|&node| node != "ZZZ")
            .count() + 1
    }

    fn loop_time(
            mut node: String,
            route: &str,
            network: &HashMap<String, HashMap<char, String>>) -> usize {

        let mut visited: HashMap<String, usize> = HashMap::new();

        for (time,  dir) in route.chars().cycle().enumerate() {
            if node.chars().last() == Some('Z') {
                if visited.contains_key(&(node.to_string())) {
                    return time - visited[&(node.to_string())]
                }

                visited.insert(node.to_string(), time);
            }
            node = network[&node][&dir].clone();
        }

        unreachable!()
    }

    pub fn solve_b(input: &str) -> usize {
        let (route, network) = parse(input);

        network.keys()
            .filter(|s| s.chars().last().unwrap() == 'A')
            .cloned()
            .map(|start| loop_time(start, &route, &network) / route.len())
            .product::<usize>() * route.len()
    }

    #[cfg(test)]
    mod test_day08 {
        use super::*;

        static INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        static INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT1), 2);
            assert_eq!(solve_a(INPUT2), 6);
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

    println!("Solution A-part: {}", day08::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day08::solve_b(&buffer.trim()));
}
