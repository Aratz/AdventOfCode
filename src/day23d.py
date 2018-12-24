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

lside = 9

points = [[x, y, z ]
        for x in np.arange(
            klim['x'][0], klim['x'][1] + 1,
            max(1, klen['x']//lside))
        for y in np.arange(
            klim['y'][0], klim['y'][1] + 1,
            max(1, klen['y']//lside))
        for z in np.arange(
            klim['z'][0], klim['y'][1] + 1, 
            max(1, klen['z']//lside))
        ]

best = sorted(points, key=lambda p: cost(p))[:lside**2]
print(best)
fact = 2

for i in range(int(round(np.log(max(klen.values()))/np.log(fact)))):
    points = []
    for point in best:
        points.extend(
        [
            [
                npr.randint(point[0] - klen['x']//fact**(i+1), point[0] + klen['x']//fact**(i+1) + 1),
                npr.randint(point[1] - klen['y']//fact**(i+1), point[1] + klen['y']//fact**(i+1) + 1),
                npr.randint(point[2] - klen['z']//fact**(i+1), point[2] + klen['z']//fact**(i+1) + 1),
                ]
            for x in range(lside)
            for y in range(lside)
            for z in range(lside)
            ])

    best = sorted(points, key=lambda p: cost(p))[:lside**2]
    print([(p, cost(p)) for p in best])
    print()
