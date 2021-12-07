#!/usr/bin/env python3

import itertools
import sys

last_c = None
first_c = None
tot = 0
for c in itertools.chain.from_iterable(sys.stdin):
    if not c.isdigit():
        continue
    d = int(c)
    if not first_c:
        first_c = c
    if c == last_c:
        tot += d
    last_c = c
if last_c == first_c:
    tot += int(last_c)
print(tot)
