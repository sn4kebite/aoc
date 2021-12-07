import sys

instructions = [line.rstrip() for line in sys.stdin]

pc = 0
regs = {
    'a': 0,
    'b': 0,
    'c': 1,
    'd': 0,
}

def getvalue(n):
    try:
        return int(n, 10)
    except ValueError:
        return regs[n]

while pc < len(instructions):
    args = instructions[pc].split()
    op = args[0]
    if op == 'cpy':
        regs[args[2]] = getvalue(args[1])
    elif op == 'inc':
        regs[args[1]] += 1
    elif op == 'dec':
        regs[args[1]] -= 1
    elif op == 'jnz':
        if getvalue(args[1]):
            pc += getvalue(args[2])
            continue
    else:
        raise RuntimeError('Unknown op %s' % op)
    pc += 1
print(regs)
