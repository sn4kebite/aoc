import sys

def print_display():
    print('\n'.join(''.join('#' if c else '.' for c in line) for line in display))

display = [[False]*50 for _ in range(6)]

for line in sys.stdin:
    cmd, params = line.rstrip().split(None, 1)
    if cmd == 'rect':
        w, h = map(int, params.split('x'))
        for y in range(h):
            display[y][:w] = [True]*w
    elif cmd == 'rotate':
        params = params.split()
        which = params.pop(0)
        if which == 'column':
            x = int(params.pop(0).split('=')[1])
            n = int(params[-1])
            for i in range(n):
                orig = [display[y][x] for y in range(6)]
                for y in range(6):
                    display[y][x] = orig[(y - 1) % 6]
        elif which == 'row':
            y = int(params.pop(0).split('=')[1])
            n = int(params[-1])
            for i in range(n):
                orig = list(display[y][x] for x in range(50))
                for x in range(50):
                    display[y][x] = orig[(x - 1) % 50]
        else:
            raise RuntimeError('Invalid rotate type "%s"' % which)
    else:
        raise RuntimeError('Invalid command "%s"' % cmd)

print_display()
print(sum(sum(line) for line in display))
