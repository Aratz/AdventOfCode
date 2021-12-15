class Fighter:
    def __init__(self, team, pos, hp=200):
        self.pid = hash(pos)
        self.team = team
        self.pos = pos
        self.hp = 200
        self.power = 3

    def mark_adj(self, c, cave):
        x, y = pos
        targets = []
        for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            try:
                cave[y + dy][x + dx] = c if cave[y + dy][x + dx] == '.' else cave[y + dy][x + dx]
                targets.append((y + dy, x + dx))
            except IndexError:
                continue
        return targets

    def closest_target(self, cave):
        enemy = 'E' if self.team == 'G' else 'G'
        queue = [self.pos]
        targets = []
        while queue:
            y, x = queue.pop(0)

            if isinstance(cave[y][x], int):
                continue

            visited = []
            new = []
            for (dy, dx) in [(-1, 0), (0, -1), (0, 1), (1, 0)]:
                try:
                    if isinstance(cave[y + dy][x + dx], int):
                        visited.append(cave[y + dy][x + dx])
                    elif cave[y + dy][x + dx] == '.':
                        new.append((y + dy, x + dx))
                except IndexError:
                    continue

            cave[y][x] = min(visited) + 1 if visited else 0
            queue.extend(new)

            for (dy, dx) in [(-1, 0), (0, -1), (0, 1), (1, 0)]:
                try:
                    if cave[y + dy][x + dx] == enemy:
                        targets.append((cave[y][x], (y + dy, x + dx)))
                except IndexError:
                    continue

        return min(targets)

    def attack(self, enemy):
        enemy.hp -= self.power

def clean(cave):
    for y, row in enumerate(cave):
        for x, c in enumerate(row):
            if isinstance(c, int):
                cave[y][x] = '.'

from sys import stdin

cave = []
fighters = {}

for y, line in enumerate(stdin):
    line = line.strip()
    cave.append([c for c in line])
    for x, c in enumerate(line):
        if c in ['E', 'G']:
            fighters[hash((y, x))] = Fighter(c, (y, x))

nround = 0
gameover = False
while not gameover:
    round_order = sorted(fighters.values(), key=lambda f: f.pos)
    for current in round_order:
        if current.hp < 0:
            continue
        y, x = current.pos
        enemy = 'E' if current.team == 'G' else 'G'

        #check if remaining targets
        if not [f for f in fighters.values() if f.team==enemy]:
            gameover = True
            nround -= 1
            break

        #move
        try:
            target_coord = current.closest_target(cave)
        except ValueError:
            continue
        finally:
            cave[y][x] = current.team
            clean(cave)

        if target_coord[0] > 0:
            target = [f for f in fighters.values() if f.pos == target_coord[1]][0]
            target.closest_target(cave)
            yt, xt = target.pos
            cave[yt][xt] = target.team

            candidate_move = []
            for (dy, dx) in [(-1, 0), (0, -1), (0, 1), (1, 0)]:
                try:
                    if isinstance(cave[y + dy][x + dx], int):
                        candidate_move.append((cave[y + dy][x + dx], (y + dy, x + dx)))
                except IndexError:
                    continue
            clean(cave)
            yn, xn = min(candidate_move)[1]
            cave[yn][xn] = current.team
            cave[y][x] = '.'
            current.pos = (yn, xn)
            y, x = yn, xn

        #attack
        targets = []
        for (dy, dx) in [(-1, 0), (0, -1), (0, 1), (1, 0)]:
            try:
                if cave[y + dy][x + dx] == enemy:
                    targets.append([f for f in fighters.values()
                            if f.pos == (y + dy, x + dx)][0])
            except IndexError:
                continue
        try:
            target = min(targets, key=lambda f: (f.hp, f.pos))
            current.attack(target)
            if target.hp <= 0:
                yt, xt = target.pos
                cave[yt][xt] = '.'
            continue
        except ValueError:
            pass
    if gameover:
        break

    dead = []
    for k, v in fighters.items():
        if v.hp <= 0:
            dead.append(k)
    for k in dead:
        del fighters[k]

    nround += 1
    print(nround)
    for row in cave:
        print(''.join([str(c%10) if isinstance(c, int) else c for c in row]))
    for f in fighters.values():
        print(f.team, f.hp, f.pos)

print(sum([f.hp for f in fighters.values()]) * nround)
