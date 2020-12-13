from sys import stdin

offset = 200
pots = offset*"." + input().split(": ")[1] + offset*"."
input()

rules = {}

for rule in stdin:
    if not rule:
        break
    rule = rule.split(" => ")
    rules[rule[0]] = rule[1].strip()

print(0, pots)
for n in range(122):
    pots = ".." + "".join([rules.get(pots[i:i+5], '.') for i in range(len(pots) - 4)]) + ".."
    print(n+1, pots)

print(sum([ i - offset for i, c in enumerate(pots) if c == '#']) + (50*10**9-122)*len([c for c in pots if c == '#']))
