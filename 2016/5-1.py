import itertools
import hashlib
import sys

password = ''
door_id = open('5-input', 'r').read().strip()
for i in itertools.count():
    b = '{}{}'.format(door_id, i).encode()
    s = hashlib.md5(b).hexdigest()
    if s[:5] == '00000':
        password += s[5]
        sys.stdout.write('\r{}{}'.format(password, '*' * (8 - len(password))))
        sys.stdout.flush()
        if len(password) == 8:
            break
print('\rPassword: {}'.format(password))
