mod day11 {
    use std::collections::VecDeque;
    use std::collections::HashMap;

    const ITEMS: usize = 15;

    fn is_valid(state: &[i32; ITEMS]) -> bool {
        let valid_range = state.iter().all(|&v| 0 <= v && v < 4);
        //i even, not 0
        let no_frying = (0..(ITEMS-1)/2).map(|i| i*2 + 2)
            .all(|chip|
                 state[chip] == state[chip-1]
                 || (0..(ITEMS-1)/2).map(|j| j*2 + 1).all(|gen| state[gen] != state[chip]));

        valid_range && no_frying
    }

    /// Elevator is at index 0
    /// Generators are at odds indices
    /// Chips are at even indices
    pub fn solve_a(init: [i32; ITEMS]) -> i32 {
        let mut queue: VecDeque<([i32; ITEMS], i32)> = VecDeque::new();
        let mut visited: HashMap<[i32; ITEMS], i32> = HashMap::new();

        queue.push_back((init, 0));

        while let Some((state, dist)) = queue.pop_front() {
            if visited.contains_key(&state) { continue; }

            visited.insert(state, dist);
            for move1 in [-1, 1] {
                for item1 in 1..ITEMS {
                    if state[item1] != state[0] { continue; }

                    let mut new_state = state.clone();
                    new_state[0] += move1;
                    new_state[item1] += move1;
                    if is_valid(&new_state)
                        && !visited.contains_key(&new_state) {
                        queue.push_back((new_state.clone(), dist + 1));
                    }

                    for item2 in 1..ITEMS {
                        if state[item2] != state[0]
                            || item1 == item2
                            || state[item2] != state[item1] 
                            || (item1%2 == 1 && item2%2 == 0 && item1 + 1 != item2)
                            || (item2%2 == 1 && item1%2 == 0 && item2 + 1 != item1)
                            {
                            continue;
                        }
                        let mut new_state2 = new_state.clone();
                        new_state2[item2] += move1;
                        if is_valid(&new_state2)
                            && !visited.contains_key(&new_state2) {
                            queue.push_back((new_state2.clone(), dist + 1));
                        }
                    }
                }
            }

            if visited.contains_key(&[3; ITEMS]) { break; }
        }

        visited[&[3; ITEMS]]
    }

}

fn main() {
    let init = [0, 0, 0, 0, 0, 1, 2, 1, 1, 1, 1, 3, 3, 3, 3];
    println!("Solution A-part: {}", day11::solve_a(init));
    let init = [0, 0, 0, 0, 0, 1, 2, 1, 1, 1, 1, 0, 0, 0, 0];
    println!("Solution B-part: {}", day11::solve_a(init));

}
