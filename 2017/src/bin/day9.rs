use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut level = 0;
    let mut score = 0;
    let mut garbage = false;
    let mut skip = false;
    for c in line.chars() {
        if skip {
            skip = false;
            continue;
        }

        if c == '!' {
            skip = true;
            continue;
        }

        if !garbage {
            match c {
                '{' => { level += 1; },
                '}' => { score += level; level -= 1; },
                '<' => { garbage = true; },
                _ => {},
            }
        }
        else {
            if c == '>' {
                garbage = false;
            }
        }
    }

    println!("{}", score);
}
