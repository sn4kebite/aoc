import itertools
import re
import sys

ops = []

def op(r):
    def wrapper(f):
        ops.append((re.compile(r), f))
    return wrapper

@op(r'^swap position (\d+) with position (\d+)$')
def swap_pos(m, data):
    x, y = map(int, m.groups())
    data[x], data[y] = data[y], data[x]
    return data

@op(r'^swap letter (\w) with letter (\w)$')
def swap_letter(m, data):
    a, b = m.groups()
    x = data.index(a)
    y = data.index(b)
    data[x], data[y] = data[y], data[x]
    return data

@op(r'^rotate (\w+) (\d+) steps?$')
def rotate_steps(m, data):
    d = 1 if m.group(1) == 'left' else -1
    steps = int(m.group(2))
    return data[d * steps % len(data):] + data[:d * steps % len(data)]

@op(r'^rotate based on position of letter (\w)$')
def rotate_pos(m, data):
    l = m.group(1)
    i = data.index(l) + 1
    if i >= 5:
        i += 1
    i = -i
    return data[i % len(data):] + data[:i % len(data)]

@op(r'^reverse positions (\d+) through (\d+)$')
def reverse_pos(m, data):
    x, y = map(int, m.groups())
    return data[:x] + list(reversed(data[x:y+1])) + data[y+1:]

@op(r'^move position (\d+) to position (\d+)$')
def move_pos(m, data):
    x, y = map(int, m.groups())
    data.insert(y, data.pop(x))
    return data

commands = [line.rstrip() for line in sys.stdin]
result = sys.argv[1]
for p in itertools.permutations('abcdefgh'):
    data = list(p)
    for line in commands:
        for r, f in ops:
            m = r.match(line)
            if m:
                data = f(m, data)
                break
        else:
            raise RuntimeError('Invalid command {}'.format(line))
    data = ''.join(data)
    p = ''.join(p)
    if data == result:
        print(p)
        break
