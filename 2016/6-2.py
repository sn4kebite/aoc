import sys

lines = [line.strip() for line in sys.stdin]
s = [dict() for x in lines[0]]
for line in lines:
    for i, c in enumerate(line):
        s[i].setdefault(c, 0)
        s[i][c] += 1
print(''.join(sorted(x.items(), key = lambda x: x[1])[0][0] for x in s))
