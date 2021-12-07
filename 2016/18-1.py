import sys

data = sys.stdin.read().rstrip()
rows = int(sys.argv[1])
traps = data.count('.')

for row in range(1, rows):
    new_data = ''
    for i in range(len(data)):
        trap = False
        if i == 0:
            trap = trap or data[i:i+2] in ('^^', '.^')
        elif i > 0:
            trap = trap or data[i-1:i+2] in ('^^.', '^..')
        if i == len(data)-1:
            trap = trap or data[i-1:i+1] in ('^^', '^.')
        elif i < len(data)-1:
            trap = trap or data[i-1:i+2] in ('.^^', '..^')
        new_data += '^' if trap else '.'
    data = new_data
    traps += data.count('.')
print(traps)
