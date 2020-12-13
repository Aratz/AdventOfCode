use std::io::{self, BufRead};

fn main(){
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut garbage = false;
    let mut skip = false;
    let mut garbage_size = 0;
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
            if c == '<' {
                garbage = true;
            }
        }
        else {
            if c == '>' {
                garbage = false;
            }
            else {
                garbage_size += 1;
            }
        }
    }

    println!("{}", garbage_size);
}
