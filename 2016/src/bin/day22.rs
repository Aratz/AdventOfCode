extern crate regex;

mod day22 {
    use regex::Regex;

    #[derive(Debug)]
    struct Disk {
        x: i32,
        y: i32,
        size: i32,
        used: i32,
        avail: i32,
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

            Disk { x, y, size, used, avail }
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
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let disks: Vec<String> = stdin.lock().lines().skip(2)
        .map(|l| l.unwrap()).collect();

    println!("Solution A-part: {}", day22::solve_a(&disks));
}
