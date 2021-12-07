import collections
import itertools
import re
import sys

seen_states = set()

class Base(object):
    __slots__ = ['name']

    def __init__(self, name):
        if isinstance(name, str):
            name = get_key(name)
        #if isinstance(self, Microchip):
        #    name = -name
        self.name = name

    #def __lt__(self, other):
    #    return self.name < other.name

    #def __radd__(self, other):
    #    return self.name + other

    def __str__(self):
        return '%s %s' % (self.__class__.__name__, self.name)

    def __repr__(self):
        return '"%s"' % self

    def __hash__(self):
        return hash(self.name)

    def __eq__(self, other):
        return self.__class__ == other.__class__ and self.name == other.name

class Generator(Base):
    pass

class Microchip(Base):
    pass

class Floor(object):
    __slots__ = ['generators', 'microchips']

    def __init__(self, generators, microchips):
        self.generators = generators
        self.microchips = microchips

    #def __hash__(self):
    #    return hash((self.generators, self.microchips, sum(self.generators), sum(self.microchips)))

    #def __eq__(self, other):
    #    print('eq')
    #    return len(self.generators) == len(other.generators) and sum(self.generators + self.microchips) == sum(other.generators + other.microchips)
    #    #return self.generators == other.generators and self.microchips == other.microchips

    def __len__(self):
        return len(self.generators) + len(self.microchips)

    def __repr__(self):
        return 'Floor(%d generators, %d microchips)' % (len(self.generators), len(self.microchips))

    def copy(self):
        return Floor(self.generators.copy(), self.microchips.copy())

class State(object):
    __slots__ = ['floors', 'position', 'step']

    def __init__(self, floors, position, step):
        self.floors = [x.copy() for x in floors]
        self.position = position
        self.step = step

    def gen_state(self):
        states = []
        for i, n in enumerate(names):
            g, m = None, None
            for fi, f in enumerate(self.floors):
                if any(i == x.name for x in f.generators):
                    g = fi
                if any(i == x.name for x in f.microchips):
                    m = fi
            if g is None or m is None:
                raise RuntimeError('wat')
            states.append((g, m))
        #states.sort()
        return (self.position, tuple(states))

    def __hash__(self):
        return hash(self.gen_state())
        #return hash((tuple(self.floors), self.position))

    #def __eq__(self, other):
    #    print('state eq')
    #    return all(self.floors[i] == other.floors[i] for i in range(4)) and self.position == other.position

    def __str__(self):
        s = ''
        for f in reversed(self.floors):
            s += '{} {}\n'.format(f.generators, f.microchips)
        return s

    @property
    def floor(self):
        return self.floors[self.position]

    def is_valid(self):
        for f in self.floors:
            if len(f.generators):
                for m in f.microchips:
                    if not any(g.name == m.name for g in f.generators):
                        return False
        return True

    def copy(self):
        return State(self.floors, self.position, self.step + 1)

    def move(self, generators=(), microchips=(), up = True):
        state = self.copy()
        last_position = state.position
        if up:
            state.position += 1
        else:
            state.position -= 1
        for g in generators:
            #temp = set(state.floors[last_position].generators)
            #temp.remove(g)
            #state.floors[last_position].generators = frozenset(temp)
            #temp = set(state.floor.generators)
            #temp.add(g)
            #state.floor.generators = frozenset(temp)
            state.floors[last_position].generators.remove(g)
            state.floor.generators.add(g)
        for m in microchips:
            #temp = set(state.floors[last_position].microchips)
            #temp.remove(m)
            #state.floors[last_position].microchips = frozenset(temp)
            #temp = set(state.floor.microchips)
            #temp.add(m)
            #state.floor.microchips = frozenset(temp)
            state.floors[last_position].microchips.remove(m)
            state.floor.microchips.add(m)
        return state

    def done(self):
        return sum(len(f) for f in self.floors[:3]) == 0

    def score(self):
        return sum(len(f) * 10**i for i, f in enumerate(self.floors, 1))

    def get_moves(self):
        e = tuple(self.floor.generators) + tuple(self.floor.microchips)
        if not len(e):
            print('banan')
        one = itertools.combinations(e, 1)
        two = itertools.combinations(e, 2)
        combinations = itertools.chain(two, one)
        moves = []
        for c in combinations:
            generators = tuple(g for g in c if isinstance(g, Generator))
            microchips = tuple(m for m in c if isinstance(m, Microchip))
            if len(generators) + len(microchips) != len(c):
                print('wtf')
            if self.position < 3:
                state = self.move(generators, microchips)
                if hash(state) not in seen_states and state.is_valid():
                    moves.append(state)
            #if self.position > 0 and not all(len(f) == 0 for f in self.floors[:self.position]):
            if self.position > 0:
                state = self.move(generators, microchips, False)
                if hash(state) not in seen_states and state.is_valid():
                    moves.append(state)
        moves.sort(key = State.score, reverse = True)
        return moves

def f(state):
    #print(state)
    #print(state.floor)
    #print(state.floor.generators, len(state.floor.generators), sum(state.floor.generators))
    #print(state.floor.microchips, len(state.floor.microchips), sum(state.floor.microchips))
    queue = collections.deque(state.get_moves())
    #print(state)
    #i = 0
    while len(queue):
        #i += 1
        #print(queue)
        #print('i={}, queue={}, seen_states={}'.format(i, len(queue), len(seen_states)))
        s = queue.popleft()
        #print(s.gen_state())
        if hash(s) in seen_states:
            continue
        seen_states.add(hash(s))
        #print(s)
        #if i == 3:
        #    break
        if s.done():
            print('done after {} steps'.format(s.step))
            return
        moves = s.get_moves()
        queue.extend(moves)

floors = []

names = []
def get_key(name):
    if not name in names:
        names.append(name)
    return names.index(name)

for line in sys.stdin:
    generators = set(map(Generator, re.findall(r'(\w+) generator', line)))
    microchips = set(map(Microchip, re.findall(r'(\w+)-compatible microchip', line)))
    floors.append(Floor(generators, microchips))
f(State(floors, 0, 0))
