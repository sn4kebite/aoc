import itertools
import hashlib
import re
import sys

salt = sys.stdin.read().rstrip()

keys = 0

for i in itertools.count():
    if keys == 64:
        break
    key = hashlib.md5((salt + str(i)).encode('ascii')).hexdigest()
    m = re.search(r'(.)(\1{2})', key)
    if m:
        triplet = m.group(1)[0]*5
        for j in range(i+1, i+1001):
            key = hashlib.md5((salt + str(j)).encode('ascii')).hexdigest()
            if triplet in key:
                keys += 1
                print('Key {} on index {}'.format(keys, i))
                break
