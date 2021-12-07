import re
import sys

names = []
def get_key(name):
    if not name in names:
        names.append(name)
    return names.index(name)

floors = []
for floor, line in enumerate(sys.stdin):
    generators = set(map(get_key, re.findall(r'(\w+) generator', line)))
    microchips = set(map(get_key, re.findall(r'(\w+)-compatible microchip', line)))
    floors.append((generators, microchips))
state = []
for n in range(len(names)):
    g, m = None, None
    for fi, f in enumerate(floors):
        if n in f[0]:
            g = fi
        if n in f[1]:
            m = fi
    state.append((g, m))
print(floors)
print(state)
