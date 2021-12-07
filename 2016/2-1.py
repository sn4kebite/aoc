import sys

d = 5

for line in sys.stdin:
    for c in line.strip():
        if c == 'U' and d > 3:
            d -= 3
        elif c == 'L' and d % 3 != 1:
            d -= 1
        elif c == 'D' and d < 7:
            d += 3
        elif c == 'R' and d % 3 != 0:
            d += 1
    sys.stdout.write(str(d))
print()
