mod day12 {
    use std::collections::{VecDeque, HashMap};

    #[derive(Clone)]
    struct Path {
        path: Vec<String>,
        vcount: HashMap<String, usize>,
    }

    impl Path {
        fn new() -> Self {
            Self { path:vec![], vcount: HashMap::new() }
        }

        fn push(&mut self, v: String) {
            if v.to_lowercase() == v {
                *self.vcount.entry(v.to_string()).or_default() += 1;
            }

            self.path.push(v);
        }

        fn is_reachable(&self, v: &str, max_v: usize) -> bool {
            v.to_uppercase() == v
                || !self.vcount.contains_key(v)
                || (self.vcount.values().all(|&count| count < max_v)
                    && v != "start" && v != "end")
        }
    }

    fn generate_graph(edges: &[(String, String)]) -> HashMap<String, Vec<String>> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for (v1, v2) in edges {
            graph.entry(v1.to_string()).or_default().push(v2.to_string());
            graph.entry(v2.to_string()).or_default().push(v1.to_string());
        }

        graph
    }

    fn pathfinder(graph: &HashMap<String, Vec<String>>, max_v: usize) -> Vec<Vec<String>> {
        let mut queue = VecDeque::new();
        queue.push_back(("start".to_string(), Path::new()));

        let mut complete_paths = Vec::new();

        while let Some((v, mut path)) = queue.pop_front() {
            if !path.is_reachable(&v, max_v) { continue; }

            path.push(v.to_string());

            if &v  == "end" {
                complete_paths.push(path.path);
                continue;
            }

            for v2 in &graph[&v] {
                queue.push_back((v2.to_string(), path.clone()));
            }
        }

        complete_paths
    }

    pub fn solve_a(edges: &[(String, String)]) -> usize {
        let graph = generate_graph(edges);

        pathfinder(&graph, 1).len()
    }

    pub fn solve_b(edges: &[(String, String)]) -> usize {
        let graph = generate_graph(edges);

        pathfinder(&graph, 2).len()
    }

    #[cfg(test)]
    mod test_day12 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let edges = vec![
                ("start".to_string(),"A".to_string()),
                ("start".to_string(),"b".to_string()),
                ("A".to_string(),"c".to_string()),
                ("A".to_string(),"b".to_string()),
                ("b".to_string(),"d".to_string()),
                ("A".to_string(),"end".to_string()),
                ("b".to_string(),"end".to_string()),
            ];

            assert_eq!(solve_a(&edges), 10);
        }

        #[test]
        fn test_solve_b() {
            let edges = vec![
                ("start".to_string(),"A".to_string()),
                ("start".to_string(),"b".to_string()),
                ("A".to_string(),"c".to_string()),
                ("A".to_string(),"b".to_string()),
                ("b".to_string(),"d".to_string()),
                ("A".to_string(),"end".to_string()),
                ("b".to_string(),"end".to_string()),
            ];

            assert_eq!(solve_b(&edges), 36);
        }
    }
}

fn main () {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let edges: Vec<(String, String)> = stdin.lock().lines().
        map(|l| {
            let edge: Vec<String> = l.unwrap().split('-')
                .map(|s| s.to_string()).collect();
            (edge[0].to_string(), edge[1].to_string())
        }).collect();

    println!("Solution A-part: {}", day12::solve_a(&edges));
    println!("Solution B-part: {}", day12::solve_b(&edges));
}
