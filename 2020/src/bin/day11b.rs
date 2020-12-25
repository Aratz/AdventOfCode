use std::io::{self, BufRead};

#[derive(Clone)]
struct WaitingArea {
    seats: Vec<Vec<char>>,
}

impl PartialEq for WaitingArea {
    fn eq(&self, other: &Self) -> bool {
        self.seats == other.seats
    }
}

impl Eq for WaitingArea {}

#[allow(dead_code)]
impl WaitingArea {
    fn are_valid_coord(&self, i: i32, j: i32) -> bool {
        i >= 0
            && j >= 0
            && i < self.seats.len() as i32
            && j < self.seats[i as usize].len() as i32
    }

    fn count_neighbors(&self, i: usize, j: usize)
      -> usize {
        let mut count = 0;

        let i = i as i32;
        let j = j as i32;

        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 { continue }
                for k in 1.. {
                    if self.are_valid_coord(i + k*di, j + k*dj) {
                        if self.seats[(i + k*di) as usize][(j + k*dj) as usize] == '#' {
                            count += 1;
                            break;
                        }
                        else if self.seats[(i + k*di) as usize][(j + k*dj) as usize] == 'L' {
                            break;
                        }
                    }
                    else { break; }
                }
            }
        }

        count
    }

    fn update(&mut self) {
        let mut new_area = self.seats.clone();
        for (i, line) in self.seats.iter().enumerate() {
            for (j, &seat) in line.iter().enumerate() {
                if seat == 'L'
                    && self.count_neighbors(i, j) == 0 {
                    new_area[i][j] = '#';
                }
                else if seat == '#'
                    && self.count_neighbors(i, j) >= 5 {
                    new_area[i][j] = 'L';
                }
            }
        }

        self.seats = new_area;
    }

    fn count_occupied(&self) -> usize {
        self.seats.iter()
            .map(|line| line.iter().filter(|&&seat| seat == '#').count())
            .sum()
    }

    fn print_area(&self) {
        for line in self.seats.iter() {
            println!("{}", line.iter()
                     .map(|seat| seat.to_string())
                     .collect::<Vec<String>>()
                     .join(""));
        }
        println!();
    }

    fn print_neighbors(&self) {
        for (i, line) in self.seats.iter().enumerate() {
            for (j, _) in line.iter().enumerate() {
                print!("{}", self.count_neighbors(i, j));
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();

    let mut waiting_area = WaitingArea {
        seats: stdin.lock().lines()
            .map(|line| line.unwrap()
                 .chars()
                 .collect()
                 )
            .collect(),
    };

    loop {
        let old_area = waiting_area.clone();
        waiting_area.update();

        if old_area == waiting_area {
            break;
        }
    }

    println!("{}", waiting_area.count_occupied());
}
