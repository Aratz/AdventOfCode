import sys
import parse


XMIN = 400
XMAX = 650
YMAX = 2000

underground = [['' for x in range(XMIN, XMAX)] for y in range(YMAX)]

YMAX = 0
YMIN = 100

for line in sys.stdin:
    if line[0] == 'x':
        floor = parse.parse("x={:d}, y={:d}..{:d}", line)
        for y in range(floor[1], floor[2]+1):
            underground[y][floor[0] - XMIN] = '#'
        YMAX = max(YMAX, floor[2])
        YMIN = min(YMIN, floor[1])
    else:
        wall = parse.parse("y={:d}, x={:d}..{:d}", line)
        for x in range(wall[1], wall[2]+1):
            underground[wall[0]][x - XMIN] = '#'

YMAX += 1

def explore_down(start):
    y, x = start
    while y < YMAX - 1 and not underground[y+1][x]:
        y += 1
        underground[y][x] = '|'

    if y >= YMAX - 1:
        return

    leak_left = explore_side((y, x), -1)
    leak_right = explore_side((y, x), 1)

    while y >= start[0] and not leak_left and not leak_right:
        xl = x - 1
        while underground[y][xl] == '|':
            underground[y][xl] = '~'
            xl -= 1

        xr = x + 1
        while underground[y][xr] == '|':
            underground[y][xr] = '~'
            xr += 1

        underground[y][x] = '~'
        y -= 1
        leak_left = explore_side((y, x), -1)
        leak_right = explore_side((y, x), 1)

def explore_side(start, direction):
    y, x = start
    while True:
        if not underground[y + 1][x]:
            underground[y + 1][x] = '|'
            explore_down((y + 1, x))
        elif not underground[y][x + direction] and underground[y + 1][x] in ['~', '#']:
            x += direction
        else:
            break
        underground[y][x] = '|'

    return underground[y + 1][x] == '|'

explore_down((0, 500 - XMIN))

for row in underground[YMIN:YMAX]:
    print(''.join([c if c else ' ' for c in row]))

print(sum([row.count('~') + row.count('|') for row in underground[YMIN:YMAX]]))
