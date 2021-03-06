from sys import stdin

cart_symbols = ['>', 'v', '<', '^']
tracks_c = []
tracks_empty = []

carts = []

for y, row in enumerate(stdin):
    row = row[:-1]
    tracks_c.append([c for c in row])
    tracks_empty.append([c for c in row.replace('>', '-').replace('<', '-')
            .replace('^', '|').replace('v', '|')])
    for (x, c) in[(x, c) for (x, c) in enumerate(row) if c in cart_symbols]:
        carts.append((y, x, 'l'))

turns = {
    'l': {
        '>' : '^',
        '^': '<',
        '<': 'v',
        'v':'>',
        },
    's': {
        '>' : '>',
        '^': '^',
        '<': '<',
        'v': 'v',
        },
    'r': {
        '>': 'v',
        '^': '>',
        '<': '^',
        'v': '<',
    }
}

movement = {
        '>': (0, 1),
        '^': (-1, 0),
        '<': (0, -1),
        'v': (1, 0),
        }

turn_order = {
        'l': 's',
        's': 'r',
        'r': 'l',
        }

tick = 0
while True:
    carts.sort()

    crashed_carts = []
    for i, (y, x, turn) in enumerate(carts):
        if i in crashed_carts:
            continue
        m_y, m_x = movement[tracks_c[y][x]]
        cart = tracks_c[y][x]
        if tracks_c[y + m_y][x + m_x] in cart_symbols:
            crashed_carts.append(i)
            j = [j for j, c in enumerate(carts)
                    if c[0] == y + m_y and c[1] == x + m_x][0]
            crashed_carts.append(j)
            tracks_c[y][x] = tracks_empty[y][x]
            tracks_c[y + m_y][x + m_x] = tracks_empty[y + m_y][x + m_x]
            continue

        if tracks_empty[y + m_y][x + m_x] == '/':
            if cart in ['>', '<']:
                tracks_c[y + m_y][x + m_x] = turns['l'][cart]
            else:
                tracks_c[y + m_y][x + m_x] = turns['r'][cart]
        elif tracks_empty[y + m_y][x + m_x] == '\\':
            if cart in ['>', '<']:
                tracks_c[y + m_y][x + m_x] = turns['r'][cart]
            else:
                tracks_c[y + m_y][x + m_x] = turns['l'][cart]
        elif tracks_empty[y + m_y][x + m_x] == '+':
            tracks_c[y + m_y][x + m_x] = turns[turn][cart]
            turn = turn_order[turn]
        else:
            tracks_c[y + m_y][x + m_x] = cart

        tracks_c[y][x] = tracks_empty[y][x]
        carts[i] = (y + m_y, x + m_x, turn)

    for i in reversed(sorted(crashed_carts)):
        carts.pop(i)
    if len(carts) == 1:
        print("{},{}".format(carts[0][1], carts[0][0]))
        break
    tick += 1
