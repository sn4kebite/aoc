#!/usr/bin/env python3

import sys

tot = 0
input_str = sys.stdin.read().strip()
for i, c in enumerate(input_str):
    if not c.isdigit():
        continue
    d = int(c)
    if input_str[(i + len(input_str) // 2) % len(input_str)] == c:
        tot += d
print(tot)
