import sys

forest = [[a for a in line.strip()] for line in sys.stdin]

for line in forest:
    print(''.join(line))
print()
for t in range(10):
    new_forest = [[a for a in line] for line in forest]

    for y, line in enumerate(new_forest):
        for x, acre in enumerate(line):
            trees = 0
            lumberyards = 0
            open_acres = 0
            for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1),
                    (1, 1), (-1, -1), (-1, 1), (1, -1)]:
                if y + dy < 0 or x + dx < 0:
                    continue
                try:
                    trees += 1 if forest[y + dy][x + dx] == '|' else 0
                    lumberyards += 1 if forest[y + dy][x + dx] == '#' else 0
                    open_acres += 1 if forest[y + dy][x + dx] == '.' else 0
                except IndexError:
                    continue

            if acre == '.' and trees >= 3:
                new_forest[y][x] = '|'
            elif acre == '|' and lumberyards >= 3:
                new_forest[y][x] = '#'
            elif acre == '#' and not (lumberyards >= 1 and trees >= 1):
                new_forest[y][x] = '.'

    forest = new_forest
    for line in forest:
        print(''.join(line))
    print()

wood = sum([line.count('|') for line in forest]) * sum([line.count('#') for line in forest])
print(wood)
