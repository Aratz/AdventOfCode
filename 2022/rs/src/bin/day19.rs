extern crate regex;

mod day19 {
    use regex::Regex;
    use std::cmp::max;
    use std::collections::{VecDeque, HashSet};

    #[derive(Debug)]
    struct Blueprint {
        id: i32,
        r_cost: [[i32; 3]; 4],
    }

    fn parse(input: &str) -> Vec<Blueprint> {
        let re_blueprint = Regex::new(r"Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_ore>\d+) ore. Each clay robot costs (?P<clay_ore>\d+) ore. Each obsidian robot costs (?P<obs_ore>\d+) ore and (?P<obs_clay>\d+) clay. Each geode robot costs (?P<geo_ore>\d+) ore and (?P<geo_obs>\d+) obsidian.").unwrap();

        re_blueprint.captures_iter(input)
            .map(|capt|
                 Blueprint {
                    id: capt["id"].parse().unwrap(),
                    r_cost: [
                        [capt["ore_ore"].parse().unwrap(), 0, 0],
                        [capt["clay_ore"].parse().unwrap(), 0, 0],
                        [capt["obs_ore"].parse().unwrap(), capt["obs_clay"].parse().unwrap(), 0],
                        [capt["geo_ore"].parse().unwrap(), 0, capt["geo_obs"].parse().unwrap()]],
                 })
            .collect()
    }

    #[derive(Clone, Hash, Eq, PartialEq, Debug)]
    struct State {
        time: i32,
        robots: [i32; 4],
        resources: [i32; 4],
    }

    fn evaluate_bp(bp: &Blueprint, max_time: i32) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited: HashSet<State> = HashSet::new();

        queue.push_back(State {
            time: 0,
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0]});

        let mut max_robots:Vec<i32> = (0..3).map(
            |i| bp.r_cost.iter().map(|cost| cost[i]).max().unwrap()).collect();
        max_robots.push(i32::MAX);

        let mut max_geo = 0;

        while let Some(state) = queue.pop_front() {
            if state.time > max_time
                || visited.contains(&state)
                || state.robots.iter().zip(max_robots.iter()).any(|(n_r, max_r)| n_r > max_r)
                {
                    continue;
            }
            let State { time, robots, resources } = state;

            for i_rb in 0..robots.len() {
                let wait_time = resources.iter().enumerate()
                    .zip(bp.r_cost[i_rb].iter())
                    .map(|((i_rs, n_rs), c_rs)| (i_rs, max(0, c_rs - n_rs)))
                    .map(|(i_rs, n_needed)|
                         if robots[i_rs] > 0 { ((n_needed as f32) / (robots[i_rs] as f32)).ceil() as i32 }
                         else { (max_time + 1) * n_needed })
                    .max().unwrap();

                if wait_time + time >= max_time { continue; }

                let mut new_resources = resources.clone();
                for i_rs in 0..(new_resources.len() - 1) {
                    new_resources[i_rs] += (wait_time + 1) * robots[i_rs];
                    new_resources[i_rs] -= bp.r_cost[i_rb][i_rs];
                    assert!(new_resources[i_rs] >= 0);
                }
                new_resources[3] += (wait_time + 1) * robots[3];

                let mut new_robots = robots.clone();
                new_robots[i_rb] += 1;
                let new_time = time + wait_time + 1;

                let remaining_time = max_time - new_time;
                if resources[3]
                        + robots[3]*(remaining_time + 1)
                        + remaining_time * (remaining_time + 1) / 2
                        < max_geo {
                    continue;
                }

                let new_state = State {
                    time: new_time,
                    resources: new_resources,
                    robots: new_robots,
                };
                if !visited.contains(&new_state) {
                    queue.push_back(new_state);
                }

            }

            max_geo = max(max_geo, state.resources[3]);
            visited.insert(state);
        }
        max_geo
    }

    pub fn solve_a(input: &str) -> i32 {
        parse(input)
            .into_iter()
            .map(|bp| bp.id * evaluate_bp(&bp, 24))
            .sum()
    }

    pub fn solve_b(input: &str) -> i32 {
        parse(input)
            .into_iter()
            .take(3)
            .map(|bp| evaluate_bp(&bp, 32))
            .product()
    }

    #[cfg(test)]
    mod test_day19 {
        use super::*;

        static INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        #[test]
        fn test_solve_a() {
            assert_eq!(solve_a(INPUT), 33);
        }

        #[test]
        fn test_solve_b() {
            assert_eq!(solve_b(INPUT), 62 * 56);
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

    println!("Solution A-part: {}", day19::solve_a(&buffer.trim()));
    println!("Solution B-part: {}", day19::solve_b(&buffer.trim()));
}
