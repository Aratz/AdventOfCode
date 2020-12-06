use std::io::{self, Read};
use std::collections::{HashMap, VecDeque};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let plan = &buffer[1..buffer.len()-2];

    let coord_dir = vec![
        ('W', (-1, 0)),
        ('N', (0, 1)),
        ('E', (1, 0)),
        ('S', (0, -1)),
    ].iter().map(|c| *c).collect::<HashMap<char, (i32, i32)>>();
    let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
    let mut rooms: HashMap<(i32, i32), i32> = HashMap::new();

    let mut pos = (0, 0);
    rooms.insert(pos, 0);

    for dir in plan.chars(){
        match dir {
            'W' | 'N' | 'E' | 'S' => {
                let dpos = coord_dir[&dir];
                let new_pos = (pos.0 + dpos.0, pos.1 + dpos.1);
                let dist = rooms[&pos];
                rooms.entry(new_pos).or_insert(dist + 1);

                pos = new_pos;
            },
            '(' => {
                stack.push_back(pos);
            },
            '|' => {
                pos = *stack.back().unwrap();
            },
            ')' => {
                pos = stack.pop_back().unwrap();
            },
            _ => {
                panic!("Unrecognized character!");
            },
        }
    }

    println!("{:?}", rooms.values().filter(|d| **d >= 1000).count());
}
