from sys import stdin, stderr
import copy

def left_add(v, pair):
    if type(pair) is int:
        return v + pair
    else:
        return [left_add(v, pair[0]), pair[1]]

def right_add(pair, v):
    if type(pair) is int:
        return v + pair
    else:
        return [pair[0], right_add(pair[1], v)]


def explode(pair, depth=0):
    if depth == 4 and type(pair) is list:
        return pair
    else:
        if type(pair) is not int:
            l_res = explode(pair[0], depth + 1)
            if l_res is not None:
                l, r = l_res
                if l is not None and r is not None:
                    pair[0] = 0
                if r is not None:
                    pair[1] = left_add(r, pair[1])
                    r = None
                return [l, r]

            r_res = explode(pair[1], depth + 1)
            if r_res is not None:
                l, r = r_res
                if l is not None and r is not None:
                    pair[1] = 0
                if l is not None:
                    pair[0] = right_add(pair[0], l)
                    l = None
                return [l, r]
    return None

def split(pair):
    if type(pair) is int:
        if pair >= 10:
            return [pair//2, pair - pair//2]
        else:
            return None
    else:
        l_res = split(pair[0])
        if l_res is not None:
            pair[0] = l_res
            return pair

        r_res = split(pair[1])
        if r_res is not None:
            pair[1] = r_res
            return pair

    return None


class Number():
    def __init__(self, pair):
        self.pair = pair

    def __add__(self, other):
        res = Number([copy.deepcopy(self.pair), copy.deepcopy(other.pair)])
        res.reduce()
        return res

    def reduce(self):
        while True:
            if explode(self.pair) is not None:
                continue
            if split(self.pair) is None:
                break

    def magnitude(self):
        if type(self.pair) is int:
            return self.pair
        else:
            return 3 * Number(self.pair[0]).magnitude() + 2 * Number(self.pair[1]).magnitude()

    def __repr__(self):
        return str(self.pair)

    def __eq__(self, other):
        return self.pair == other.pair

def add_up(numbers):
    res = numbers[0]
    for n in numbers[1:]:
        res += n
        res.reduce()
    return res

def solve_a(raw_numbers):
    numbers = [Number(n) for n in raw_numbers]
    return add_up(numbers).magnitude()

def solve_b(raw_numbers):
    numbers = [Number(n) for n in raw_numbers]
    magns = [
                (a + b).magnitude()
                for a in numbers for b in numbers
                if a != b]\
        + [
                (b + a).magnitude()
                for a in numbers for b in numbers
                if a != b]
    return max(magns)



def get_input():
    return [eval(line) for line in stdin]

def main():
    raw_numbers = get_input()
    print(f"Solution A-part: {solve_a(raw_numbers)}")
    print(f"Solution B-part: {solve_b(raw_numbers)}")

if __name__ == "__main__":
    main()
