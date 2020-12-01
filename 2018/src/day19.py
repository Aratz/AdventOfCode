def addr(instruction, vin):
    vin[instruction[3]] = vin[instruction[1]] + vin[instruction[2]]
    return vin

def addi(instruction, vin):
    vin[instruction[3]] = vin[instruction[1]] + instruction[2]
    return vin

def mulr(instruction, vin):
    vin[instruction[3]] = vin[instruction[1]] * vin[instruction[2]]
    return vin

def muli(instruction, vin):
    vin[instruction[3]] = vin[instruction[1]] * instruction[2]
    return vin

def banr(instruction, vin):
    vin[instruction[3]] = vin[instruction[1]] & vin[instruction[2]]
    return vin

def bani(instruction, vin):
    vin[instruction[3]] = vin[instruction[1]] & instruction[2]
    return vin
def borr(instruction, vin):
    vin[instruction[3]] = vin[instruction[1]] | vin[instruction[2]]
    return vin

def bori(instruction, vin):
    vin[instruction[3]] = vin[instruction[1]] | instruction[2]
    return vin

def setr(instruction, vin):
    vin[instruction[3]] = vin[instruction[1]]
    return vin

def seti(instruction, vin):
    vin[instruction[3]] = instruction[1]
    return vin

def gtir(instruction, vin):
    vin[instruction[3]] = 1 if instruction[1] > vin[instruction[2]] else  0
    return vin

def gtri(instruction, vin):
    vin[instruction[3]] = 1 if vin[instruction[1]] > instruction[2] else 0
    return vin

def gtrr(instruction, vin):
    vin[instruction[3]] = 1 if vin[instruction[1]] > vin[instruction[2]] else 0
    return vin

def eqir(instruction, vin):
    vin[instruction[3]] = 1 if instruction[1] == vin[instruction[2]] else 0
    return vin

def eqri(instruction, vin):
    vin[instruction[3]] = 1 if vin[instruction[1]] == instruction[2] else 0
    return vin

def eqrr(instruction, vin):
    vin[instruction[3]] = 1 if vin[instruction[1]] == vin[instruction[2]] else 0
    return vin

opcodes = {
        'addr': addr,
        'addi': addi,
        'mulr': mulr,
        'muli': muli,
        'banr': banr,
        'bani': bani,
        'borr': borr,
        'bori': bori,
        'setr': setr,
        'seti': seti,
        'gtir': gtir,
        'gtri': gtri,
        'gtrr': gtrr,
        'eqir': eqir,
        'eqri': eqri,
        'eqrr': eqrr,
        }

from sys import stdin

binding = int(input().split()[-1])
registers = [0 for _ in range(6)]
raw_instructions = [line.split() for line in stdin]
instructions = [(line[0], [0] + [int(e) for e in line[1:]]) for line in raw_instructions]
pointer = 0

while pointer < len(instructions):
    instruction = instructions[pointer]
    registers[binding] = pointer
    registers = opcodes[instruction[0]](instruction[1], registers)
    pointer = registers[binding] + 1

print(registers[0])
