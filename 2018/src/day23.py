import sys
import parse

nanobots = [parse.parse("pos=<{x:d},{y:d},{z:d}>, r={r:d}", line).named
        for line in sys.stdin]

strongest = max(nanobots, key= lambda n: n['r'])

def dist(n1, n2):
    return sum([abs(n1[k] - n2[k]) for k in ['x', 'y', 'z']])

print(len([bot
    for bot in nanobots if dist(bot, strongest) <= strongest['r']]))
