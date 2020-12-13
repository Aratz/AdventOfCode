fn addr(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = vin[instruction[1]] + vin[instruction[2]];
    vin
}

fn addi(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = vin[instruction[1]] + instruction[2];
    vin
}

fn mulr(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = vin[instruction[1]] * vin[instruction[2]];
    vin
}

fn muli(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = vin[instruction[1]] * instruction[2];
    vin
}

fn banr(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = vin[instruction[1]] & vin[instruction[2]];
    vin
}

fn bani(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = vin[instruction[1]] & instruction[2];
    vin
}
fn borr(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = vin[instruction[1]] | vin[instruction[2]];
    vin
}

fn bori(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = vin[instruction[1]] | instruction[2];
    vin
}

fn setr(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = vin[instruction[1]];
    vin
}

fn seti(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = instruction[1];
    vin
}

fn gtir(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = if instruction[1] > vin[instruction[2]] { 1 } else { 0 };
    vin
}

fn gtri(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = if vin[instruction[1]] > instruction[2] { 1 } else { 0 };
    vin
}

fn gtrr(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = if vin[instruction[1]] > vin[instruction[2]] { 1 } else { 0 };
    vin
}

fn eqir(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = if instruction[1] == vin[instruction[2]] { 1 } else { 0 };
    vin
}

fn eqri(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = if vin[instruction[1]] == instruction[2] { 1 } else { 0 };
    vin
}

fn eqrr(instruction:&Vec<usize>, mut vin: Vec<usize>) -> Vec<usize> {
    vin[instruction[3]] = if vin[instruction[1]] == vin[instruction[2]] { 1 } else { 0 };
    vin
}

fn main() {
    use std::io::{self, BufRead};

    let opcodes: Vec<Box<Fn(&Vec<usize>, Vec<usize>)->Vec<usize>>> = vec![
        Box::new(addr),
        Box::new(addi),
        Box::new(mulr),
        Box::new(muli),
        Box::new(banr),
        Box::new(bani),
        Box::new(borr),
        Box::new(bori),
        Box::new(setr),
        Box::new(seti),
        Box::new(gtir),
        Box::new(gtri),
        Box::new(gtrr),
        Box::new(eqir),
        Box::new(eqri),
        Box::new(eqrr)
    ];

    let stdin = io::stdin();
    let sep = ['[', ',', ']'];

    let mut result = 0;
    let mut lines = stdin.lock().lines();
    loop {
        let vin = lines.next().unwrap().unwrap().split(|c:char| sep.contains(&c))
            .filter_map(|x| x.trim().parse::<usize>().ok()).collect::<Vec<usize>>();
        let instruction = lines.next().unwrap().unwrap().split(" ")
            .filter_map(|x| x.trim().parse::<usize>().ok()).collect::<Vec<usize>>();
        let vout = lines.next().unwrap().unwrap().split(|c:char| sep.contains(&c))
            .filter_map(|x| x.trim().parse::<usize>().ok()).collect::<Vec<usize>>();

        if opcodes.iter().map(|f| f(&instruction, vin.clone()))
            .filter(|ref vexp| *vexp == &vout).count() >= 3 {
                result += 1;
            }

        match lines.next() {
            Some(x) => (),
            None => break,
        }
    }
    println!("{}", result);
}
