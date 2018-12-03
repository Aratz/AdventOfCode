fn main() {
    use std::io::{self, BufRead};
    use std::collections::HashMap;

    let stdin = io::stdin();

    let mut fabric = HashMap::new();

    let mut lines = Vec::new();

    for line in stdin.lock().lines() {
        let sep = ['#', '@', ',', ':', 'x'];
        let line = line.unwrap();
        let data:Vec<i32> = line.split(|c:char| sep.contains(&c))
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap()).collect();

        lines.push(data.clone());

        for x in data[1]..data[1]+data[3] {
            for y in data[2]..data[2]+data[4] {
                let count = fabric.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
    }

    println!("{}", fabric.values().filter(|&x| *x > 1).count());

    for data in lines {
        let mut overlap = false;
        for x in data[1]..data[1]+data[3] {
            for y in data[2]..data[2]+data[4] {
                if fabric[&(x, y)] > 1 {
                    overlap = true;
                    break;
                }
            }
            if overlap {
                break;
            }
        }
        if !overlap {
            println!("{}", data[0]);
            break;
        }
    }

}
