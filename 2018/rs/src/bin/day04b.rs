fn main() {
    use std::io::{self, BufRead};
    use std::collections::HashMap;

    let stdin = io::stdin();

    let mut guards = HashMap::new();

    let mut gid = 0;
    let mut sleep = 0;

    for line in stdin.lock().lines() {
        let sep = ['[', '-', ' ', ':', ']'];
        let line = line.unwrap();
        let line = line.split("] ").collect::<Vec<&str>>();
        let tstamp = line[0].split(|c:char| sep.contains(&c))
            .collect::<Vec<&str>>();
        let action = line[1].split(" ").collect::<Vec<&str>>();


        gid = if action[0] == "Guard" { action[1][1..].parse().unwrap() }
            else { gid };

        sleep = if action[0] == "falls" { tstamp[5].parse().unwrap() }
            else { sleep };

        if action[0] == "wakes" {
                let awake = tstamp[5].parse().unwrap();
                let timeline = guards.entry(gid).or_insert(vec![0;60]);
                for t in sleep..awake {
                    timeline[t] += 1;
                }
        }
    }

    let sleepy_guard = guards.iter()
        .map(|(k, v)| (k, v.iter().enumerate().max_by_key(|&(m, x)| x).unwrap()))
        .max_by_key(|&(id, (m, x))| x).unwrap();
    println!("id:{} min:{}", sleepy_guard.0, (sleepy_guard.1).0);
    println!("{}", sleepy_guard.0 * (sleepy_guard.1).0);
}
