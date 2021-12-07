import sys

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
    x += dx * n
    y += dy * n
print(x + y)
