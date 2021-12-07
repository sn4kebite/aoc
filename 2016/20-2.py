import sys

ranges = []

for line in sys.stdin:
    start, end = map(int, line.rstrip().split('-'))
    skip = False
    for s, e in reversed(ranges):
        merge = False
        if s <= start <= e+1:
            merge = True
            start = s
        if s-1 <= end <= e:
            merge = True
            end = e
        if start <= s and end >= e:
            merge = True
        if s <= start and e >= end:
            skip = True
            break
        if merge:
            #print('merge ({}, {})'.format(start, end))
            ranges.remove((s, e))
            #break
    if skip:
        continue
    #print('adding', (start, end))
    ranges.append((start, end))
    #ranges.sort()
    #print(ranges)

ips = 0
for start, end in ranges:
    ips += end - start + 1
print(ips)
print(2**32 - ips)
