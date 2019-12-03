def dist(p1, p2):
    return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])

def convert_to_coordinates(moves, start=(0, 0)):
    res = [start]
    for move in moves:
        current_pos = res[-1]
        direction = move[0]
        dist = int(move[1:]) * (1 if direction in ['R', 'U'] else -1)
        res.append(
                (
                    current_pos[0] + (dist if direction in ['L', 'R'] else 0),
                    current_pos[1] + (dist if direction in ['U', 'D'] else 0),
                )
            )
    return res

wires = [convert_to_coordinates(input().split(',')) for _ in range(2)]

min_dist = None

dist1 = 0

for section1 in zip(wires[0], wires[0][1:]):
    dist2 = 0
    for section2 in zip(wires[1], wires[1][1:]):
        if section1[0][0] == section1[1][0]: # horizontal section
            x, y1 = section1[0]
            _, y2 = section1[1]

            x1, y = section2[0]
            x2, _ = section2[1]
        else: #vertical section
            x1, y = section1[0]
            x2, _ = section1[1]

            x, y1 = section2[0]
            _, y2 = section2[1]

        x1, x2 = min(x1, x2), max(x1, x2)
        y1, y2 = min(y1, y2), max(y1, y2)

        if x1 < x < x2 and y1 < y < y2:
            dist_inter = dist1 + dist(section1[0], (x, y))\
                       + dist2 + dist(section2[0], (x, y))

            try:
                min_dist = min(min_dist, dist_inter)
            except TypeError:
                min_dist = dist_inter

        dist2 += dist(*section2)
    dist1 += dist(*section1)

print(min_dist)

