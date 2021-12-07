import sys

#class Elf(object):
#    __slots__ = ('number', 'presents')
#
#    def __init__(self, number):
#        self.number = number
#        self.presents = 1
#
#    def __repr__(self):
#        return '<Elf({}, {})>'.format(self.number, self.presents)
#
#data = [Elf(i) for i in range(1, int(sys.stdin.read().rstrip())+1)]
#
#p = 0
#while len(data) > 1:
#    if data[p].presents == 0:
#        data.pop(p)
#        #sys.stdout.write('\r{}'.format(len(data)))
#        #sys.stdout.flush()
#        p = p % len(data)
#        continue
#    pn = (p + 1) % len(data)
#    data[p].presents += data[pn].presents
#    data[pn].presents = 0
#    p = pn
##print()
#print(data[0].number)

number = int(sys.stdin.read().rstrip())

def f(n):
    if n == 1 or n == 2:
        return 1
    i = 1
    for n in range(3, n+1):
        i += 2
        m = (n+1) if n % 2 else n
        i = i % m
    return i
print(f(number))
