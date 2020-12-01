def distance_to_COM(data, p):
    if p == 'COM':
        return 0
    else:
        return 1 + distance_to_COM(data, data[p])

direct_orbits = {}
graph = {}

while True:
    try:
        s = input()
        a, b = s.split(')')
        direct_orbits[b] = a
        graph[a] = graph.get(a, []) + [b]
        graph[b] = graph.get(b, []) + [a]
    except EOFError:
        break

dist_to_you = {p:None for p in graph}

current_pos, stop = direct_orbits['YOU'], direct_orbits['SAN']

dist_to_you[current_pos] = 0
queue = [current_pos]


while queue:
    p = queue[0]
    for dest in graph[p]:
        if dist_to_you[dest] is None:
            dist_to_you[dest] = dist_to_you[p] + 1
            queue.append(dest)

    del queue[0]

print(dist_to_you[stop])
