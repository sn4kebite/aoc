import sys

visited = set()
x = y = 0
d = 0

s = sys.stdin.read()
s = s.split(', ')
for i in s:
    t = 1 if i[0] == 'R' else -1
    n = int(i[1:])
    d = (d + t) % 4
    dx = dy = 0
    if d == 0:
        dy = 1
    elif d == 1:
        dx = 1
    elif d == 2:
        dy = -1
    elif d == 3:
        dx = -1
    for i in range(1, n+1):
        p = (x + dx * i, y + dy * i)
        if p in visited:
            print(abs(p[0]) + abs(p[1]))
            sys.exit(0)
        visited.add(p)
    x += dx * n
    y += dy * n
print(':(')
