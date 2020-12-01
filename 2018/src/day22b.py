depth = int(input().split()[1])
xt, yt = [int(e) for e in input().split()[1].split(',')]
dymax = 500
dxmax = 500

xm, ym = (0, 0)

erosion_level = [[0 for y in range(ym, yt + dymax)] for y in range(xm, xt + dxmax)]

for x in range(xm, xt + dxmax):
    for y in range(ym, yt + dymax):
        if not x and not y or x == xt and y == yt:
            erosion_level[x][y] = 0
        elif x and not y:
            erosion_level[x][y] = (x * 16807 + depth) % 20183
        elif not x and y:
            erosion_level[x][y] = (y * 48271 + depth) % 20183
        else:
            erosion_level[x][y] = (erosion_level[x - 1][y]
                    * erosion_level[x][y - 1] + depth) % 20183

risk_level = [[e % 3 for e in row] for row in erosion_level]

import heapq

MAX = 1000000

d = {(x, y, t): MAX
        for x in range(xm, xt + dxmax)
        for y in range(ym, yt + dymax)
        for t in range(3)}

start = (xm, ym, 1) # Torch cannot go through risk one areas
d[start] = 0

x, y, t = start

q = []
pred = {}

for (dx, dy, dt) in [ (1, 0, 0), (-1, 0, 0), (0, 1, 0),
        (0, -1, 0), (0, 0, 1), (0, 0, 2)]:
    n = (x + dx, y + dy, (t + dt)%3)

    if not n in d:
        continue
    if dt and (t + dt) % 3 == risk_level[x][y]:
        continue
    if (not dx or not dy) and t == risk_level[x + dx][y + dy]:
        continue

    if dt == 0:
        d[n] = d[start] + 1 #no change of tool
    else:
        d[n] = d[start] + 7

    pred[n] = start
    q.append((d[n], n))

heapq.heapify(q)

done = set([start])

while q:
    c = heapq.heappop(q)[1]
    done.add(c)
    x, y, t = c
    for (dx, dy, dt) in [ (1, 0, 0), (-1, 0, 0), (0, 1, 0),
            (0, -1, 0), (0, 0, 1), (0, 0, 2)]:
        n = (x + dx, y + dy, (t + dt)%3)

        if not n in d:
            continue
        if dt and (t + dt) % 3 == risk_level[x][y]:
            continue
        if (not dx or not dy) and t == risk_level[x + dx][y + dy]:
            continue
        if n in done:
            continue

        if dt == 0:
            dd = 1
        else:
            dd = 7

        if d[c] + dd < d[n]:
            d[n] = min(d[c] + dd, d[n])
            pred[n] = c
            heapq.heappush(q, (d[n], n))

print(d[(xt, yt, 1)])
