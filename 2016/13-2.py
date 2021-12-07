import collections
import sys

magic_number = int(sys.stdin.read().strip())

def is_wall(x, y):
    v = x*x + 3*x + 2*x*y + y + y*y
    v += magic_number
    b = 0
    for i in range(32):
        if (v >> i) & 1:
            b += 1
    return bool(b % 2)

#for y in range(10):
#    print(''.join('#' if is_wall(x, y) else '.' for x in range(10)))

class State(object):
    def __init__(self, x, y, step):
        self.x = x
        self.y = y
        self.step = step

    def __repr__(self):
        return '<State({}, {})>'.format(self.x, self.y)

    def __hash__(self):
        return hash((self.x, self.y))

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def get_moves(self):
        moves = []
        if self.x > 0 and not is_wall(self.x-1, self.y):
            moves.append(State(self.x-1, self.y, self.step+1))
        if not is_wall(self.x+1, self.y):
            moves.append(State(self.x+1, self.y, self.step+1))
        if self.y > 0 and not is_wall(self.x, self.y-1):
            moves.append(State(self.x, self.y-1, self.step+1))
        if not is_wall(self.x, self.y+1):
            moves.append(State(self.x, self.y+1, self.step+1))
        return (m for m in moves if not m in visited)

queue = collections.deque()
queue.append(State(1, 1, 0))

visited = set()

while len(queue):
    state = queue.popleft()
    visited.add(state)
    if state.step == 50:
        continue
    queue.extend(state.get_moves())
print('visited: {}'.format(len(visited)))
