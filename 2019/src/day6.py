def distance_to_COM(data, p):
    if p == 'COM':
        return 0
    else:
        return 1 + distance_to_COM(data, data[p])

direct_orbits = {}

while True:
    try:
        s = input()
        a, b = s.split(')')
        direct_orbits[b] = a
    except EOFError:
        break

print(sum([distance_to_COM(direct_orbits, p) for p in direct_orbits]))
