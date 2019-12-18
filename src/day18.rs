use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque, HashSet};

fn bfs(map: &Vec<Vec<char>>, start: (usize, usize)) -> HashMap<char, (u32, i32)> {
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0, 0));
    visited.insert(start);

    while let Some(((x, y), d, keys)) = queue.pop_front() {
        visited.insert((x, y));
        let c = map[y][x];
        if 'a' <= c && c <= 'z' {
            dist.insert(c, (d, keys));
        }

        // No need for boundary check since the area is enclosed with walls
        for &(x_n, y_n) in &[(x-1, y), (x+1, y), (x, y-1), (x, y+1)] {
            let c_n = map[y_n][x_n];
            if  c_n != '#' && !visited.contains(&(x_n, y_n)) {
                //if current spot is a door, add neighbor with extra key
                queue.push_back(((x_n, y_n), d + 1, keys | if 'A' <= c && c <= 'Z' {(1 as i32) << (c as u8 - 'A' as u8)} else { 0 }));
            }
        }
    }

    dist
}

fn shortest_path(
    map: &Vec<Vec<char>>, dist: &HashMap<char, HashMap<char, (u32, i32)>>,
    pos: char, keys: i32,
    data: &mut HashMap<(char, i32), u32>,
    ) {

    if data.contains_key(&(pos, keys)) {
        return;
    }

    //println!("{:?} {:?}", pos, keys);

    for (&next_key, &(d, nec_keys)) in dist[&pos].iter()
        .filter(|(&key, &(_d, nec_keys))| nec_keys & keys == nec_keys // I have the keys to get there
                && keys & (1 as i32) << (key as u8 - 'a' as u8) == 0 // This is a new key
                ) {
        shortest_path(map, dist, next_key, keys | (1 as i32)<<(next_key as u8 - 'a' as u8), data);
    }

    let res = match dist[&pos].iter()
        .filter(|(&key, &(_d, nec_keys))| nec_keys & keys == nec_keys // I have the keys to get there
                && keys & (1 as i32) << (key as u8 - 'a' as u8) == 0 // This is a new key
                )
        .map(|(&next_key, &(d, _nec_keys))| d + data[&(next_key, keys | (1 as i32)<<(next_key as u8 - 'a' as u8))])
        .min() {
            Some(d) => d,
            None => 0,
        };
    data.insert((pos, keys), res);
}

fn main() {
    let stdin = io::stdin();
    let map = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let pos = map.iter().enumerate()
        .flat_map(|(y, row)| row.iter().enumerate()
            .filter_map(move |(x, c)| match c {
                '@' => Some((x, y)),
                _ => None,
            })).next().unwrap();

    let keys = map.iter().enumerate()
        .flat_map(|(y, row)| row.iter().enumerate()
            .filter_map(move |(x, c)| match c {
                '@' | 'a'..='z'=> Some((x, y)),
                _ => None,
            })).collect::<Vec<(usize, usize)>>();

    let mut dist: HashMap<char, HashMap<char, (u32, i32)>> = HashMap::new();

    for &(x, y) in keys.iter() {
        dist.insert(map[y][x], bfs(&map, (x, y)));
    }

    let mut data = HashMap::new();

    shortest_path(&map, &dist, '@', 0, &mut data);

    println!("{}", data[&('@', 0)]);


}
