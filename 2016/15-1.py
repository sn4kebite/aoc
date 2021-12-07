import itertools
import re
import sys

class Disc(object):
    def __init__(self, number, positions, position):
        self.number = number
        self.positions = positions
        self.position = position

    def get_position(self, time):
        return (self.position + time) % self.positions

discs = []

for line in sys.stdin:
    number, positions, time, position = map(int, re.match(r'^Disc #(\d+) has (\d+) positions; at time=(\d+), it is at position (\d+).$', line.rstrip()).groups())
    discs.append(Disc(number, positions, (position + positions - time) % positions))

for time in itertools.count():
    #print('time', time)
    disc_time = time
    b = True
    for disc in discs:
        #print('disc={}, position={}'.format(disc.number, disc.get_position(time + disc.number)))
        if not disc.get_position(time + disc.number) == 0:
            b = False
            break
    if b:
        print('time', time)
        break
