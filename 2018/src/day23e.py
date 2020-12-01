import sys
import parse

nanobots = [parse.parse("pos=<{x:d},{y:d},{z:d}>, r={r:d}", line).named
        for line in sys.stdin]
strongest = max(nanobots, key= lambda n: n['r'])

def dist_xx(x1, x2):
    return sum([abs(x1[i] - x2[i]) for i in range(3)])

def dist_xn(x, n):
    return sum([abs(x[i] - n[k]) for i, k in enumerate(['x', 'y', 'z'])])

def dist_nn(n1, n2):
    return sum([abs(n1[k] - n2[k]) for k in ['x', 'y', 'z']])

klim = {k:(min(nanobots, key=lambda n: n[k])[k], max(nanobots, key=lambda n: n[k])[k])
        for k in ['x', 'y', 'z']}

klen = {k:klim[k][1] - klim[k][0] for k in klim}

import numpy as np
import numpy.random as npr

def cost(x):
    return (sum([1 if dist_xn(x, n) > n['r'] else 0 for n in nanobots]), dist_xx(x, [0, 0, 0]))

start = [25525018, 30692493, 30653896]

r = 100
points = [[x, y, z ]
        for x in range(start[0] - r, start[0] + 1)
        for y in range(start[1] - r, start[1] + 1)
        for z in range(start[2] - r, start[2] + 1)
        ]

best = sorted(points, key=lambda p: cost(p))[:10]
print(best)
