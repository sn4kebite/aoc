import sys

valid = 0
triangles = []
for line in sys.stdin:
    a, b, c = [int(x) for x in line.strip().split()]
    triangles.append((a, b, c))
    if len(triangles) < 3:
        continue
    for i in range(3):
        a, b, c = triangles[0][i], triangles[1][i], triangles[2][i]
        if a + b > c and a + c > b and b + c > a:
            valid += 1
    triangles[:] = []
print(valid)
