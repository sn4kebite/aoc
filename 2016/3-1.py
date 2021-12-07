import sys

valid = 0
for line in sys.stdin:
    a, b, c = [int(x) for x in line.strip().split()]
    if a + b > c and a + c > b and b + c > a:
        valid += 1
print(valid)
