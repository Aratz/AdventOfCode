extern crate num_integer;

use std::io::{self, BufRead};
use num_integer:: Integer;

struct Moon {
    coord: (i64, i64, i64),
    v: (i64, i64, i64),
}

#[allow(dead_code)]
impl Moon {
    fn x(&self) -> i64 {
        self.coord.0
    }

    fn vx(&self) -> i64 {
        self.v.0
    }

    fn y(&self) -> i64 {
        self.coord.1
    }

    fn vy(&self) -> i64 {
        self.v.1
    }

    fn z(&self) -> i64 {
        self.coord.2
    }

    fn vz(&self) -> i64 {
        self.v.2
    }

    fn potential_energy(&self) -> i64 {
        self.coord.0.abs() + self.coord.1.abs() + self.coord.2.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.v.0.abs() + self.v.1.abs() + self.v.2.abs()
    }

    fn update_pos(&mut self) {
        self.coord.0 +=  self.v.0;
        self.coord.1 +=  self.v.1;
        self.coord.2 +=  self.v.2;
    }

    fn update_v(&mut self, dv: (i64, i64, i64)) {
        self.v.0 += dv.0;
        self.v.1 += dv.1;
        self.v.2 += dv.2;
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn print(&self) {
        println!("pos=<x= {}, y= {}, z= {}>, vel=<x= {}, y= {}, z= {}>",
              self.x(), self.y(), self.z(),
              self.vx(), self.vy(), self.vz(),
              );
    }
}

fn main() {
    let stdin = io::stdin();

    let mut moons = stdin.lock().lines()
        .map(|l| {
            let coord = l.unwrap().split(' ').map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            Moon {coord: (coord[0], coord[1], coord[2]), v: (0, 0, 0)}
        })
        .collect::<Vec<Moon>>();

    let init_state = (
        (moons[0].x(), moons[1].x(), moons[2].x(), moons[3].x(),
            moons[0].vx(), moons[1].vx(), moons[2].vx(), moons[3].vx()),
        (moons[0].y(), moons[1].y(), moons[2].y(), moons[3].y(),
            moons[0].vy(), moons[1].vy(), moons[2].vy(), moons[3].vy()),
        (moons[0].z(), moons[1].z(), moons[2].z(), moons[3].z(),
            moons[0].vz(), moons[1].vz(), moons[2].vz(), moons[3].vz()),
        );

    let mut s = 0;

    let (mut rx, mut ry, mut rz) = (0 as u64, 0 as u64, 0 as u64);

    while rx == 0 || ry == 0 || rz == 0 {
        s += 1;
        let dv = moons.iter()
            .map(|m1| moons.iter()
                 .map(|m2| (
                         (m2.x() - m1.x()).signum(),
                         (m2.y() - m1.y()).signum(),
                         (m2.z() - m1.z()).signum(),
                         ))
                 .fold((0, 0, 0), |sum, i| (
                         sum.0 + i.0,
                         sum.1 + i.1,
                         sum.2 + i.2,
                         ))).collect::<Vec<(i64, i64, i64)>>();

        for i in 0..4 {
            moons[i].update_v(dv[i]);
            moons[i].update_pos();
        }

        let state = (
            (moons[0].x(), moons[1].x(), moons[2].x(), moons[3].x(),
                moons[0].vx(), moons[1].vx(), moons[2].vx(), moons[3].vx()),
            (moons[0].y(), moons[1].y(), moons[2].y(), moons[3].y(),
                moons[0].vy(), moons[1].vy(), moons[2].vy(), moons[3].vy()),
            (moons[0].z(), moons[1].z(), moons[2].z(), moons[3].z(),
                moons[0].vz(), moons[1].vz(), moons[2].vz(), moons[3].vz()),
            );

        if rx == 0 && init_state.0 == state.0 {
            rx = s;
        }
        if ry == 0 && init_state.1 == state.1 {
            ry = s;
        }
        if rz == 0 && init_state.2 == state.2 {
            rz = s;
        }

        if s%100_000 == 0 {
            println!("Searching (s: {}): {} {} {}", s, rx, ry, rz);
        }
    }
    println!("{} {} {}", rx, ry, rz);
    println!("{}", rx.lcm(&ry).lcm(&rz));
}
