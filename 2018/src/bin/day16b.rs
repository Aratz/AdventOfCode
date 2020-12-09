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
        Box::new(banr),//4 0
        Box::new(eqrr),//15 1
        Box::new(setr),//8 2
        Box::new(eqir),//13 3
        Box::new(bori),//7 4
        Box::new(muli),//3 5
        Box::new(bani),//5 6
        Box::new(borr),//6 7
        Box::new(gtir),//10 8
        Box::new(gtrr),//12 9
        Box::new(addi),//1 10
        Box::new(gtri),//11 11
        Box::new(eqri),//14 12
        Box::new(addr),//0 13
        Box::new(mulr),//2 14
        Box::new(seti),//9 15
    ];

    let stdin = io::stdin();

    let mut registers = vec![0, 0, 0, 0];
    for line in stdin.lock().lines() {
        let instruction = line.unwrap().split(" ")
            .filter_map(|x| x.trim().parse::<usize>().ok()).collect::<Vec<usize>>();

        registers = opcodes[instruction[0]](&instruction, registers);
    }
    println!("{}", registers[0]);
}
