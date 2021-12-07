import collections
import hashlib
import sys

data = sys.stdin.read().rstrip()

class State(object):
    def __init__(self, pos=(0,0), path=''):
        self.pos = pos
        self.path = path

    def get_moves(self):
        h = hashlib.md5('{}{}'.format(data, self.path).encode('ascii')).hexdigest()
        moves = [d for x, d in zip(h[:4], 'UDLR') if x in 'bcdef']
        for m in moves:
            if m == 'U' and self.pos[1] > 0:
                yield State((self.pos[0], self.pos[1] - 1), self.path + m)
            elif m == 'D' and self.pos[1] < 3:
                yield State((self.pos[0], self.pos[1] + 1), self.path + m)
            elif m == 'L' and self.pos[0] > 0:
                yield State((self.pos[0] - 1, self.pos[1]), self.path + m)
            elif m == 'R' and self.pos[0] < 3:
                yield State((self.pos[0] + 1, self.pos[1]), self.path + m)

queue = collections.deque()
queue.append(State())
l = 0
while len(queue):
    state = queue.popleft()
    if state.pos == (3, 3):
        l = len(state.path)
        continue
    queue.extend(state.get_moves())
print(l)
