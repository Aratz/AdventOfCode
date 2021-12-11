extern crate regex;

mod day22 {
    use regex::Regex;
    use std::cmp::Reverse;
    use std::collections::{VecDeque, BinaryHeap, HashSet, HashMap};

    #[derive(PartialEq, Clone, Copy)]
    enum NodeType {
        Small,
        Data,
        Big,
        Empty
    }

    #[allow(dead_code)]
    struct Disk {
        x: i32,
        y: i32,
        size: i32,
        used: i32,
        avail: i32,
        node_type: NodeType,
    }

    fn parse_disks(disks: &[String]) -> Vec<Disk> {
        let reg = Regex::new(r"/dev/grid/node-x(?P<x>\d+)-y(?P<y>\d+)\s+(?P<size>\d+)T\s+(?P<used>\d+)T\s+(?P<avail>\d+)T").unwrap();
        disks.iter().map(|s| {
            let capt = reg.captures(s).unwrap();
            let x = capt["x"].parse().unwrap();
            let y = capt["y"].parse().unwrap();
            let size = capt["size"].parse().unwrap();
            let used = capt["used"].parse().unwrap();
            let avail = capt["avail"].parse().unwrap();

            let node_type = match used {
                u if u == 0 => NodeType::Empty,
                u if u <= 100 => NodeType::Small,
                _ => NodeType::Big,
            };

            Disk { x, y, size, used, avail, node_type }
        }).collect()
    }

    pub fn solve_a(disks: &[String]) -> usize {
        let disks = parse_disks(disks);

        disks.iter()
            .flat_map(|a| disks.iter().map(move |b| (a, b)))
            .filter(|(a, b)| a.used > 0
                    && (a.x, a.y) != (b.x, b.y)
                    && a.used < b.avail)
            .count()
    }

    fn assemble_grid(disks: &[String])
        -> (Vec<Vec<NodeType>>, (i32, i32), (i32, i32)) {
        let disks = parse_disks(disks);

        let max_x = disks.iter().map(|d| d.x).max().unwrap();
        let max_y = disks.iter().map(|d| d.y).max().unwrap();

        let mut grid: Vec<Vec<NodeType>> = (0..=max_y)
            .map(|_| (0..=max_x).map(|_| NodeType::Empty).collect())
            .collect();

        let mut empty = (0, 0);
        let mut data = (0, 0);

        for disk in disks {
            grid[disk.y as usize][disk.x as usize] = if disk.x == max_x && disk.y == 0 {
                data = (disk.x, disk.y);
                NodeType::Data
            }
            else { disk.node_type };

            if grid[disk.y as usize][disk.x as usize] == NodeType::Empty { empty = (disk.x, disk.y); }
        }

        (grid, empty, data)
    }

    fn is_valid((x, y): (i32, i32), grid: &[Vec<NodeType>]) -> bool {
        0 <= x && x < grid[0].len() as i32
     && 0 <= y && y < grid.len() as i32
    }

    fn cost(
        start: (i32, i32),
        end: (i32, i32),
        data: (i32, i32),
        grid: &[Vec<NodeType>]
        ) -> i32 {
        let neighbors = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((start, 0));

        while let Some((pos, d)) = queue.pop_front() {
            if visited.contains(&pos) { continue; }

            if pos == end { return d; }

            visited.insert(pos);

            let (x, y) = pos;
            for (dx, dy) in neighbors.iter() {
                if is_valid((x + dx, y + dy), grid)
                    && (x + dx, y + dy) != data
                    && grid[(y + dy) as usize][(x + dx) as usize] != NodeType::Big {
                    queue.push_back(((x + dx, y + dy), d + 1));
                }
            }
        }

        unreachable!();
    }

    pub fn solve_b(disks: &[String]) -> i32 {
        let (grid, empty, data) = assemble_grid(disks);

        let target = (0, 0);

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), (data, empty)));

        let mut visited = HashMap::new();

        let neighbors = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((Reverse(d), (data, empty))) = queue.pop() {
            if visited.contains_key(&(data, empty)) { continue; }

            if data == target { return d; }

            visited.insert((data, empty), d);

            let (x, y) = data;
            for (dx, dy) in neighbors.iter() {
                if is_valid((x + dx, y + dy), &grid)
                    && grid[(y + dy) as usize][(x + dx) as usize] != NodeType::Big {
                    queue.push((
                            Reverse(d + cost(empty, (x + dx, y + dy), data, &grid) + 1),
                            ((x + dx, y + dy), data)));
                }
            }
        }

        unreachable!();
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let disks: Vec<String> = stdin.lock().lines().skip(2)
        .map(|l| l.unwrap()).collect();

    println!("Solution A-part: {}", day22::solve_a(&disks));
    println!("Solution B-part: {}", day22::solve_b(&disks));
}
