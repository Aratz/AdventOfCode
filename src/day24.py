import sys
import parse

boost = 42

immune = {}
infection = {}
for i,line in enumerate(sys.stdin):
    agent = parse.parse(
            "{units:d} units each with {hp:d} hit points{weakim}with an attack that does {attack:d} {attack_type} damage at initiative {init:d}\n", line).named
    agent['immune'] = []
    agent['weak'] = []
    for weakim in agent['weakim'].replace('(', '').replace(')', '').strip().split(';'):
        wk = parse.parse("weak to {}", weakim.strip())
        if wk:
            agent['weak'] = wk.fixed[0].split(', ')
        im = parse.parse("immune to {}", weakim.strip())
        if im:
            agent['immune'] = im.fixed[0].split(', ')

    agent['id'] = i
    agent['threat'] = None
    if i < 10:
        agent['team'] = 'immune'
        agent['attack'] += boost
        immune[i] = agent
    else:
        agent['team'] = 'infection'
        infection[i] = agent

def damage(a, d):
    return a['units'] * a['attack'] * (
            (0 if a['attack_type'] in d['immune'] else 1)
            * (2 if a['attack_type'] in d['weak'] else 1))

while immune and infection:
    # target selection
    print("target")
    for agent in sorted(list(immune.values()) + list(infection.values()),
            key=lambda a: (-a['units']*a['attack'], -a['init'])):

        try:
            target = sorted([enemy for enemy in list(immune.values()) + list(infection.values())
                    if agent['team'] != enemy['team'] and not enemy['threat']],
                    key=lambda a: (-damage(agent, a), -a['units']*a['attack'], -a['init']))
            target = target[0]
            if damage(agent, target) > 0:
                target['threat'] = agent
            else:
                target = None
        except IndexError:
            target = None
        agent['target'] = target
        print(agent['id'], agent['units']*agent['attack'], (target['id'], damage(agent, agent['target'])) if target else None)
    print()

    print('attack')
    # attack
    for agent in sorted(list(immune.values()) + list(infection.values()),
            key=lambda a: -a['init']):
        if agent['units'] > 0 and agent['target']:
            print(agent['id'], agent['target']['id'], damage(agent, agent['target']) // agent['target']['hp'])
            agent['target']['units'] -= damage(agent, agent['target']) // agent['target']['hp']
        agent['threat'] = None

    dead = [agent['id'] for agent in immune.values() if agent['units'] <= 0]
    for k in dead:
        del immune[k]
    dead = [agent['id'] for agent in infection.values() if agent['units'] <= 0]
    for k in dead:
        del infection[k]

if immune:
    print('Victory!')
else:
    print('Defeat :(')
print(sum([agent['units'] for agent in list(immune.values()) + list(infection.values())]))
