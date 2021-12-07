import math
import sys

number = int(sys.stdin.read().rstrip())

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
#data = [Elf(i) for i in range(1, number+1)]
#
#p = 0
#while len(data) > 1:
#    pn = (p + len(data) // 4 * 3) % len(data)
#    data[p].presents += data[pn].presents
#    data[pn].presents = 0
#    data.pop(pn)
#    p = pn % len(data)
##print()
#print(data[0].number)

def f(n):
    #return n, (n % 4), ((n // 4) % 3), (n // 4), (1 + (n // 4) % 3) % n
    #return n // 4, n % 4, math.floor(math.log10(n)), n - (n // 4) * (n % 4) - math.floor(math.log10(n))
    #return n // 2, n % 4, math.floor(math.log(n)), n - (n // 2) * (n % 4) - math.floor(math.log(n))
    #r = 1 + (n + math.floor(math.log(n, 2)))
    r = 1 + 2*n #* math.floor(math.log(n, 2))
    return r, r % n
print(f(number))
