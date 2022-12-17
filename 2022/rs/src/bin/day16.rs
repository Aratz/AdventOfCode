extern crate regex;

mod day16 {
    use regex::Regex;
    use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
    use std::cmp::{Ordering, min, max};

    type Graph = HashMap<String, (i32, Vec<String>)>;
    type WGraph = HashMap<String, (i32, HashMap<String, i32>)>;

    fn parse(input: &str) -> Graph {
        let re_valve = Regex::new(r"Valve (?P<valve>\w+) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<dest>(:?\w+(:?, )?)+)").unwrap();

        re_valve.captures_iter(input)
            .map(|capt| (
                    capt["valve"].to_string(),
                    (
                        capt["flow"].parse().unwrap(),
                        capt["dest"].split(", ").map(|s| s.to_string()).collect::<Vec<_>>()
                    )
                )
            )
            .collect()
    }

    fn cross_dist(origin: &str, graph: &Graph) -> HashMap<String, i32> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back((origin.to_string(), 0));
        visited.insert(origin.to_string(), 0);

        while let Some((node, dist)) = queue.pop_front() {
            for neigh in &graph[&node].1 {
                if !visited.contains_key(neigh) {
                    queue.push_back((neigh.to_string(), dist + 1));
                    visited.insert(neigh.to_string(), dist + 1);
                }
            }
        }

        visited
    }

    fn reduce_graph(graph: &Graph) -> WGraph {
        let working_valves = graph.iter()
            .filter(|(v, (fl, _))| fl > &0 || v.as_str() == "AA")
            .map(|(v, _)| v)
            .collect::<Vec<_>>();

        working_valves.iter()
            .map(|v| (
                    v.to_string(), (
                        graph[v.as_str()].0,
                        cross_dist(v, graph).into_iter()
                            .filter(|(node, _dist)|
                                    working_valves.contains(&node))
                            .collect()
                        )
                    ))
            .collect()
    }

    #[derive(Eq, PartialEq)]
    struct Path {
        unopened_valves: HashSet::<String>,
        time: i32,
        press: i32,
        last: String,
        potential_press: i32,
    }

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Path {
        fn cmp(&self, other: &Self) -> Ordering {
            self.potential_press.cmp(&other.potential_press)
        }
    }

    pub fn solve_a(input: &str) -> i32 {
        let graph = parse(input);
        let w_graph = reduce_graph(&graph);

        let nodes = w_graph.keys()
            .map(|s| s.clone())
            .collect::<Vec<_>>();

        let max_time = 30;

        let mut max_press = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Path {
            unopened_valves: nodes.iter().cloned().collect(),
            time: 0,
            press: 0,
            last: "AA".to_string(),
            potential_press: w_graph.values()
                .map(|(rate, _)| rate*max_time).sum(),
        });

        while let Some(path) = heap.pop() {
            if path.potential_press < max_press { break; }

            for node in &path.unopened_valves {
                let new_time = path.time + w_graph[&path.last].1[node] + 1;
                if new_time > max_time { continue; }

                let mut new_valves = path.unopened_valves.clone();
                new_valves.remove(node);

                let new_press = path.press + (max_time - new_time)*w_graph[node].0;

                max_press = max(max_press, new_press);

                let new_last = node.to_string();
                let new_potential = new_press + new_valves.iter()
                    .map(|n| w_graph[n].0*(max_time - new_time))
                    .sum::<i32>();

                heap.push(Path {
                    unopened_valves: new_valves,
                    time: new_time,
                    press: new_press,
                    last: new_last,
                    potential_press: new_potential,
                });
            }
        }

        max_press
    }

    #[derive(Eq, PartialEq)]
    struct Path2 {
        unopened_valves: HashSet::<String>,
        press: i32,
        potential_press: i32,
        time: [i32; 2],
        last: [String; 2],
    }

    impl PartialOrd for Path2 {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Path2 {
        fn cmp(&self, other: &Self) -> Ordering {
            self.potential_press.cmp(&other.potential_press)
        }
    }

    pub fn solve_b(input: &str) -> i32 {
        let graph = parse(input);
        let w_graph = reduce_graph(&graph);

        let nodes = w_graph.keys()
            .map(|s| s.clone())
            .collect::<Vec<_>>();

        let max_time = 26;

        let mut max_press = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Path2 {
            unopened_valves: nodes.iter()
                .filter(|n| n != &"AA")
                .cloned().collect(),
            press: 0,
            potential_press: w_graph.values()
                .map(|(rate, _)| rate*max_time).sum(),
            time: [0, 0],
            last: ["AA".to_string(), "AA".to_string()],
        });

        while let Some(path) = heap.pop() {
            if path.potential_press < max_press { break; }

            for node in &path.unopened_valves {
                let i_p = if w_graph[&path.last[0]].1[node] < w_graph[&path.last[1]].1[node]
                {
                    0
                }
                else {
                    1
                };

                let new_time = path.time[i_p] + w_graph[&path.last[i_p]].1[node] + 1;
                if new_time > max_time { continue; }

                let mut new_valves = path.unopened_valves.clone();
                new_valves.remove(node);

                let new_press = path.press + (max_time - new_time)*w_graph[node].0;

                max_press = max(max_press, new_press);

                let new_last = node.to_string();
                let new_potential = new_press + new_valves.iter()
                    .filter(|&n| min(
                        w_graph[n].1[&path.last[i_p]] + new_time,
                        w_graph[n].1[&path.last[1-i_p]] + path.time[1-i_p]) < max_time)
                    .map(|n|
                         w_graph[n].0*(
                             max_time - min(
                                w_graph[n].1[&path.last[i_p]] + new_time,
                                w_graph[n].1[&path.last[1-i_p]] + path.time[1-i_p])))
                    .sum::<i32>();

                if new_potential < max_press { continue; }

                heap.push(Path2 {
                    unopened_valves: new_valves,
                    press: new_press,
                    potential_press: new_potential,
                    time: [new_time, path.time[1-i_p]],
                    last: [new_last, path.last[1-i_p].to_string()],
                });
            }
        }

        max_press
    }

    #[cfg(test)]
    mod test_day16 {
        use super::*;

        static INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 1651);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 1707);
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

    println!("Solution A-part: {}", day16::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day16::solve_b(&buffer.trim()));
}
