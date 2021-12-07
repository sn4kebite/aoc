import itertools
import hashlib
import re
import sys

salt = sys.stdin.read().rstrip()

triplets = []
keys = []

def gen_hash(i):
    key = hashlib.md5((salt + str(i)).encode('ascii')).hexdigest()
    for j in range(2016):
        key = hashlib.md5(key.encode('ascii')).hexdigest()
    return key

for i in itertools.count():
    if len(keys) >= 64 and not any(any(t[1] < k for k in keys) for t in triplets):
        break
    key = gen_hash(i)
    remove_triplets = set()
    for triplet, triplet_i in triplets:
        if triplet in key:
            keys.append(triplet_i)
            print('Key {} on index {}'.format(len(keys), triplet_i))
            remove_triplets.add((triplet, triplet_i))
        else:
            if i - triplet_i == 1000:
                remove_triplets.add((triplet, triplet_i))
    for triplet in remove_triplets:
        triplets.remove(triplet)
    m = re.search(r'(.)(\1{2})', key)
    if m:
        triplet = m.group(1)[0]*5
        triplets.append((triplet, i))
keys.sort()
print(keys[63])
