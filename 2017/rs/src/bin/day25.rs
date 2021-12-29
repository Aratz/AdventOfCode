extern crate regex;

mod day25 {
    use regex::Regex;
    use std::collections::{HashMap, HashSet};

    #[allow(dead_code)]
    struct State {
        name: char,
        actions: [(bool, bool, char); 2],
    }

    fn parse(input: &str) -> (char, i32, HashMap<char, State>) {
        let re_state = Regex::new(r"In state (?P<state>[A-Z]):\n  If the current value is 0:\n    - Write the value (?P<write_0>\d+).\n    - Move one slot to the (?P<move_0>\w+).\n    - Continue with state (?P<state_0>[A-Z]).\n  If the current value is 1:\n    - Write the value (?P<write_1>\d+).\n    - Move one slot to the (?P<move_1>\w+).\n    - Continue with state (?P<state_1>[A-Z]).").unwrap();

        let start = input.lines().next().unwrap()
            .split(' ').last().unwrap()
            .chars().next().unwrap();

        let max_steps: i32 = input.lines().nth(1).unwrap()
            .split(' ').nth(5).unwrap()
            .parse().unwrap();

        let mut states = HashMap::new();

        for cap in re_state.captures_iter(input) {
            let name = cap["state"].chars().next().unwrap();
            let v0 = (&cap["write_0"] == "1", &cap["move_0"] == "right", cap["state_0"].chars().next().unwrap());
            let v1 = (&cap["write_1"] == "1", &cap["move_1"] == "right", cap["state_1"].chars().next().unwrap());

            states.insert(name, State { name, actions: [v0, v1] });
        }

        (start, max_steps, states)
    }

    pub fn solve(input: &str) -> usize {
        let (start, max_steps, states) = parse(input);

        let mut cursor = 0i32;
        let mut tape = HashSet::new();
        let mut state = start;

        for _ in 0..max_steps {
            let current_value = if tape.contains(&cursor) { 1 } else { 0 };

            if states[&state].actions[current_value].0 {
                tape.insert(cursor);
            }
            else {
                tape.remove(&cursor);
            }

            cursor += if states[&state].actions[current_value].1 { 1 } else { -1 };

            state = states[&state].actions[current_value].2;
        }

        tape.len()
    }

    #[cfg(test)]
    mod test_day25 {
        use super::*;

        #[test]
        fn test_solve() {
            let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

            assert_eq!(solve(&input), 3);
        }
    }
}

fn main() {
    use std::io::{self, Read};

    let mut buffer = String::new();

    let stdin = io::stdin();

    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    println!("Solution day 25: {}", day25::solve(&buffer));

}
