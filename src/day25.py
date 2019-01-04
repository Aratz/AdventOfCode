import sys

def dist(a, b):
    return sum([abs(ia - ib) for ia, ib in zip(a, b)])

nodes = {}

for i, line in enumerate(sys.stdin):
    pos = [int(e) for e in line.strip().split(',')]
    edges= []
    for k, (p, _) in nodes.items():
        if dist(pos, p) <= 3:
            edges.append(k)
            nodes[k][1].append(i)
    nodes[i] = (pos, edges)

visited = [False for _ in nodes]
constellations = 0
while nodes:
    root = list(nodes.keys())[0]
    queue = [root]
    current_constellation = []

    while queue:
        node = queue.pop(0)
        if not visited[node]:
            visited[node] = True
            queue.extend(nodes[node][1])
            current_constellation.append(node)
    constellations += 1
    for node in current_constellation:
        del nodes[node]

print(constellations)

