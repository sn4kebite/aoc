import sys

k = ('  5  ', ' 26A ', '137BD', ' 48C ', '  9  ')

d = 5
p = (0, 2)

for line in sys.stdin:
    for c in line.strip():
        if c == 'U':
            v = (0, -1)
        elif c == 'L':
            v = (-1, 0)
        elif c == 'D':
            v = (0, 1)
        elif c == 'R':
            v = (1, 0)
        np = (p[0] + v[0], p[1] + v[1])
        try:
            nd = k[np[0]][np[1]]
        except IndexError:
            continue
        if nd != ' ':
            d = nd
            p = np
    sys.stdout.write(d)
print()
