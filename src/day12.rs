use std::io::{self, BufRead};

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

    let steps = 1000;

    for _ in 0..steps {
        let dv = moons.iter().enumerate()
            .map(|(i, m1)| moons.iter().enumerate().filter(|(j, _m2)| i != *j)
                 .map(|(_i, m2)| (
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
    }

    println!("{}", moons.iter().map(|m| m.total_energy()).sum::<i64>());
}
