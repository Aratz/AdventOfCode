depth = int(input().split()[1])
xt, yt = [int(e) for e in input().split()[1].split(',')]

xm, ym = (0, 0)

erosion_level = [[0 for y in range(ym, yt + 1)] for y in range(xm, xt + 1)]

for x in range(xm, xt + 1):
    for y in range(ym, yt + 1):
        if not x and not y or x == xt and y == yt:
            erosion_level[x][y] = 0
        elif x and not y:
            erosion_level[x][y] = (x * 16807 + depth) % 20183
        elif not x and y:
            erosion_level[x][y] = (y * 48271 + depth) % 20183
        else:
            erosion_level[x][y] = (erosion_level[x - 1][y]
                    * erosion_level[x][y - 1] + depth) % 20183

risk_level = sum([sum([e % 3 for e in row]) for row in erosion_level])
print(risk_level)

