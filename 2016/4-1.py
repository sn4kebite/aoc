import re
import sys

total = 0
for line in sys.stdin:
    print(line)
    m = re.match(r'^(?P<name>[a-z]+(?:-(?:[a-z]+))*)-(?P<number>\d+)\[(?P<checksum>[a-z]+)\]$', line)
    name = m.groupdict()['name']
    number = int(m.groupdict()['number'])
    checksum = m.groupdict()['checksum']
    name = name.replace('-', '')
    cm = {}
    for c in name:
        cm.setdefault(c, 0)
        cm[c] += 1
    chars = list(cm.items())
    chars.sort(key = lambda c: (c[1], -ord(c[0])), reverse = True)
    top = ''.join(c[0] for c in chars[:5])
    print(chars, top)
    if top == checksum:
        total += number
print(total)
