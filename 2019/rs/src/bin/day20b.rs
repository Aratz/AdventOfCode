use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let stdin = io::stdin();
    let map = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut orphan_gates: HashMap<String, (usize, usize)> = HashMap::new();
    let mut connections: HashMap<(usize, usize), ((usize, usize), i32)> = HashMap::new();

    // Search for vertical gates
    for y in 0..map.len()-1 {
        for x in 0..map[y].len() {
            if 'A' <= map[y][x] && map[y][x] <= 'Z'
                && 'A' <= map[y + 1][x] && map[y + 1][x] <= 'Z' {
                let gate: String = [map[y][x], map[y + 1][x]].iter().collect();
                let y_gate = if y == 0 || (y < map.len() - 2 && map[y + 2][x] == '.')
                    { y + 2}
                    else { y - 1};
                let level = if y == 0 || y == map.len() - 2 { -1 } else { 1 };

                match orphan_gates.remove(&gate) {
                    Some(pos_gate) => {
                        connections.insert((x, y_gate), (pos_gate, level));
                        connections.insert(pos_gate, ((x, y_gate), -level));
                    },
                    None => { orphan_gates.insert(gate, (x, y_gate)); },
                }
            }
        }
    }

    // Search horizontal gates
    for x in 0..map[0].len()-1 {
        for y in 0..map.len() {
            if 'A' <= map[y][x] && map[y][x] <= 'Z'
                && 'A' <= map[y][x + 1] && map[y][x + 1] <= 'Z' {
                let gate: String = [map[y][x], map[y][x + 1]].iter().collect();
                let x_gate = if x == 0 || (x < map[0].len() - 2 && map[y][x + 2] == '.')
                    { x + 2}
                    else { x - 1};
                let level = if x == 0 || x == map[0].len() - 2 { -1 } else { 1 };

                match orphan_gates.remove(&gate) {
                    Some(pos_gate) => {
                        connections.insert((x_gate, y), (pos_gate, level));
                        connections.insert(pos_gate, ((x_gate, y), -level));
                    },
                    None => { orphan_gates.insert(gate, (x_gate, y)); },
                }
            }
        }
    }

    // BFS
    let mut queue:VecDeque::<((usize, usize), i32, usize)> = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((orphan_gates[&String::from("AA")], 0, 0));
    visited.insert((orphan_gates[&String::from("AA")], 0));

    while let Some(((x, y), l, d)) = queue.pop_front() {
        visited.insert(((x, y), l));

        if l == 0 && (x, y) == orphan_gates[&String::from("ZZ")] {
            println!("{}", d);
            break;
        }

        if connections.contains_key(&(x, y)) // It's a door
            && l + connections[&(x, y)].1 >= 0 // It's not an outer level outer door
            && !visited.contains(&(connections[&(x, y)].0, l + connections[&(x, y)].1))
            // I've not visited the otherside of this door
            {
                queue.push_back((connections[&(x, y)].0, l + connections[&(x, y)].1, d + 1));
        }
        else {
            for &(x_n, y_n) in &[(x-1, y), (x+1, y), (x, y-1), (x, y+1)] {
                let c_n = map[y_n][x_n];
                if  c_n == '.' && !visited.contains(&((x_n, y_n), l)) {
                    queue.push_back(((x_n, y_n), l, d + 1));
                }
            }
        }
    }
}
