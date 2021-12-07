import sys

s = set()
ranges = []

for line in sys.stdin:
    start, end = map(int, line.rstrip().split('-'))
    ranges.append((start, end))

ip = 0
ranges.sort()
for start, end in ranges:
    if start <= ip <= end:
        ip = end + 1
print(ip)
