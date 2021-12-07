import re
import sys

def ceasar(c, n):
    i = ord(c)
    if i < ord('a') or i > ord('z'):
        return c
    return chr(ord('a') + (i - ord('a') + n) % 26)

for line in sys.stdin:
    m = re.match(r'^(?P<name>[a-z]+(?:-(?:[a-z]+))*)-(?P<number>\d+)\[(?P<checksum>[a-z]+)\]$', line)
    name = m.groupdict()['name']
    number = int(m.groupdict()['number'])
    checksum = m.groupdict()['checksum']
    cm = {}
    for c in name.replace('-', ''):
        cm.setdefault(c, 0)
        cm[c] += 1
    chars = list(cm.items())
    chars.sort(key = lambda c: (c[1], -ord(c[0])), reverse = True)
    top = ''.join(c[0] for c in chars[:5])
    if top == checksum:
        name = ''.join(ceasar(c, number) for c in name)
        print(name.replace('-', ' '))
        if name == 'northpole-object-storage':
            print(number)
