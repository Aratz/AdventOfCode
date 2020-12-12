use std::io::BufRead;

#[derive(Debug)]
enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

#[derive(Debug)]
struct Boat {
    pos: (i32, i32),
    angle: i32,
}

impl Boat {
    fn new() -> Self {
        Boat {
            pos: (0, 0),
            angle: 0,
        }
    }

    fn move_boat(&mut self, inst: &Instruction) {
        match inst {
            Instruction::N(v) => {
                self.pos.1 += v;
            },
            Instruction::S(v) => {
                self.pos.1 -= v;
            },
            Instruction::E(v) => {
                self.pos.0 -= v;
            },
            Instruction::W(v) => {
                self.pos.0 += v;
            },
            Instruction::L(v) => {
                self.angle = (self.angle + v + 360) % 360;
            },
            Instruction::R(v) => {
                self.angle = (self.angle - v + 360) % 360;
            },
            Instruction::F(v) => {
                match self.angle {
                    0   => { self.move_boat(&Instruction::E(*v)); },
                    90  => { self.move_boat(&Instruction::N(*v)); },
                    180 => { self.move_boat(&Instruction::W(*v)); },
                    270 => { self.move_boat(&Instruction::S(*v)); },
                    _ => { panic!("Invalid angle! ({})", self.angle); },
                }
            },
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    let instructions = stdin.lock().lines()
        .map(|l| {
            let line = l.unwrap();
            let mut raw_inst = line.chars();
            let action = raw_inst.next().unwrap();
            let value = raw_inst.map(|c| c.to_string())
                .collect::<Vec<String>>().join("").parse::<i32>().unwrap();
            match action {
                'N' => Instruction::N(value),
                'S' => Instruction::S(value),
                'E' => Instruction::E(value),
                'W' => Instruction::W(value),
                'L' => Instruction::L(value),
                'R' => Instruction::R(value),
                'F' => Instruction::F(value),
                _ => { panic!("Unrecognized instruction! ({}{})", action, value); },
            }
        }).collect::<Vec<Instruction>>();

    let mut boat = Boat::new();

    for inst in instructions.iter() {
        boat.move_boat(inst);
    }

    println!("{}", boat.pos.0.abs() + boat.pos.1.abs());
}
