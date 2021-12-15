import sys
import parse

nanobots = [parse.parse("pos=<{x:d},{y:d},{z:d}>, r={r:d}", line).named
        for line in sys.stdin]
strongest = max(nanobots, key= lambda n: n['r'])
print(strongest)

def dist_xx(x1, x2):
    return sum([abs(x1[i] - x2[i]) for i in range(3)])

def dist_xn(x, n):
    return sum([abs(x[i] - n[k]) for i, k in enumerate(['x', 'y', 'z'])])

def dist_nn(n1, n2):
    return sum([abs(n1[k] - n2[k]) for k in ['x', 'y', 'z']])

klim = {k:(min(nanobots, key=lambda n: n[k])[k], max(nanobots, key=lambda n: n[k])[k])
        for k in ['x', 'y', 'z']}

import numpy as np
import numpy.random as npr

def cost(x):
    return (sum([1 if dist_xn(x, n) > n['r'] else 0 for n in nanobots]), dist_xx(x, [0, 0, 0]))

IMAX = 10**8
#speed = 0.000001
speed = 1

best = np.array([26267234,30414669,31673936])
#best = np.array([strongest[k] for k in ['x', 'y', 'z']])
current = best

cc = cost(current)
cb = cc

accepted = 0
total = 0

for i in range(IMAX):
    #candidate = current + npr.normal(scale=speed*np.array([klim[k][1] - klim[k][0]
    #    for k in ['x', 'y', 'z']]))
    candidate = current +np.array([npr.randint(-speed, speed+1) for i in range(3)])
    ca = cost(candidate)
    if klim['x'][0] > candidate[0] or candidate[0] > klim['x'][1]:
        continue
    if klim['y'][0] > candidate[1] or candidate[1] > klim['y'][1]:
        continue
    if klim['z'][0] > candidate[2] or candidate[2] > klim['z'][1]:
        continue

    alpha = cc[0]/ca[0]
    if npr.uniform() <= alpha:
        current = candidate
        cc = ca

    if cb > ca:
        best = candidate
        accepted += 1
        cb = ca
        print(best, cb)
    total += 1

    if not i%10000:
        print(current, (accepted + 1)/(total + 1))

print(best, cb)
