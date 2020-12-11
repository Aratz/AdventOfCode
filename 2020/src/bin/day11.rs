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
    fn count_neighbors(&self, i: usize, j: usize)
      -> usize {
        let mut count = 0;
        for di in 0..3 {
            for dj in 0..3 {
                if di == 1 && dj == 1 { continue }
                if di + i == 0 { continue }
                if dj + j == 0 { continue }
                if di + i - 1 == self.seats.len() { continue }
                if dj + j - 1 == self.seats[i].len() { continue }

                if self.seats[i + di - 1][j + dj - 1] == '#' {
                    count += 1;
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
                    && self.count_neighbors(i, j) >= 4 {
                    new_area[i][j] = 'L';
                }
            }
        }

        self.seats = new_area;
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

    fn count_occupied(&self) -> usize {
        self.seats.iter()
            .map(|line| line.iter().filter(|&&seat| seat == '#').count())
            .sum()
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
