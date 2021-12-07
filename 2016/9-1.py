import re
import sys

compressed = sys.stdin.read().rstrip()
total = 0
while len(compressed):
    m = re.match(r'^(\w+)', compressed)
    if m:
        length = m.span()[1]
        total += length
        print('{}: length={}'.format(compressed[:length], length))
        compressed = compressed[length:]
    elif compressed[0] == '(':
        m = re.match(r'\((\d+)x(\d+)\)', compressed)
        length = int(m.group(1))
        count = int(m.group(2))
        start = m.span()[1]
        print(compressed[:start], 'length={} count={}, start={}'.format(length, count, start))
        compressed = compressed[start+length:]
        total += count * length
    else:
        raise RuntimeError('fasdf')
print(total)
