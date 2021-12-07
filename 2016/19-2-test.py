import sys

number = int(sys.stdin.read().rstrip())

class Elf(object):
    __slots__ = ('number', 'presents')

    def __init__(self, number):
        self.number = number
        self.presents = 1

    def __repr__(self):
        return '<Elf({}, {})>'.format(self.number, self.presents)

data = [Elf(i) for i in range(1, number+1)]

p = 0
while len(data) > 1:
    #pn = (p + len(data) // 4 * 3) % len(data)
    pn = (p + len(data) // 2) % len(data)
    #print(p, pn, len(data))
    #print('{} <- {}'.format(data[p].number, data[pn].number))
    data[p].presents += data[pn].presents
    data[pn].presents = 0
    data.pop(pn)
    if p < pn:
        p += 1
    p = p % len(data)
    #p = pn % len(data)
#print()
#print(data[0].number, bin(data[0].number))
print(data[0].number)
