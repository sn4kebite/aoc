import itertools
import hashlib
import sys

password = ['_'] * 8
sys.stdout.write(''.join(password))
sys.stdout.flush()
door_id = open('5-input', 'r').read().strip()
for i in itertools.count():
    b = '{}{}'.format(door_id, i).encode()
    s = hashlib.md5(b).hexdigest()
    if s[:5] == '00000':
        pos = int(s[5], 16)
        if pos > 7 or password[pos] != '_':
            continue
        password[pos] = s[6]
        sys.stdout.write('\r{}'.format(''.join(password)))
        sys.stdout.flush()
        if all(c != '_' for c in password):
            break
print()
