import re
import sys

class Bot(object):
    def __init__(self, number):
        self.number = number
        self.low = None
        self.high = None
        self.low_dest = None
        self.high_dest = None

    def __repr__(self):
        return '<Bot(%d, %s, %s)>' % (self.number, self.low, self.high)

    def __str__(self):
        return 'bot {}'.format(self.number)

    def add(self, v):
        if self.low is None:
            self.low = v
        elif self.high is None:
            self.high = v
        else:
            raise RuntimeError('Bot %d already has two values' % self.number)
        if self.low is not None and self.high is not None and self.low > self.high:
            self.low, self.high = self.high, self.low

    @property
    def has_values(self):
        return self.low is not None and self.high is not None

    @property
    def has_destinations(self):
        return self.low_dest is not None and self.high_dest is not None

    def process(self):
        if not self.has_values or not self.has_destinations:
            return False
        if self.low == 17 and self.high == 61:
            print(self.number)
        self.low_dest.add(self.low)
        self.high_dest.add(self.high)
        self.low = None
        self.high = None
        return True

class Output(object):
    def __init__(self, number):
        self.number = number
        self.numbers = []

    def __repr__(self):
        return '<Output(%d, [%s])>' % (self.number, ', '.join(map(str, self.numbers)))

    def __str__(self):
        return 'output {}'.format(self.number)

    def add(self, v):
        self.numbers.append(v)

    def process(self):
        pass

bots = {}
outputs = {}

def get_bot(number):
    bot = bots.get(number)
    if bot is None:
        bot = bots[number] = Bot(number)
    return bot

def get_output(number):
    output = outputs.get(number)
    if output is None:
        output = outputs[number] = Output(number)
    return output

for line in sys.stdin:
    line = line.rstrip()
    m = re.match(r'^value (\d+) goes to bot (\d+)$', line)
    if m:
        value, bot_n = map(int, m.groups())
        bot = get_bot(bot_n)
        bot.add(value)
        continue
    m = re.match(r'^bot (\d+) gives (\w+) to (\w+) (\d+) and (\w+) to (\w+) (\d+)$', line)
    if m:
        from_bot_n, which_1, to_which_1, to_n_1, which_2, to_which_2, to_n_2 = m.groups()
        from_bot = get_bot(int(from_bot_n))
        for w, ww, n in ((which_1, to_which_1, int(to_n_1)), (which_2, to_which_2, int(to_n_2))):
            if ww == 'bot':
                out = get_bot(n)
            elif ww == 'output':
                out = get_output(n)
            else:
                raise RuntimeError('Invalid item "%s"' % ww)
            if w == 'low':
                from_bot.low_dest = out
            elif w == 'high':
                from_bot.high_dest = out
            else:
                raise RuntimeError('Invalid comparison "%s"' % w)
        continue
    if not m:
        raise RuntimeError('Could not parse line: %s' % line)

n = 1
i = 0
t = 0
while n:
    n = 0
    i += 1
    for b in bots.values():
        if b.process():
            t += 1
            n += 1

print('Done processing after {} iterations, {} operations'.format(i, t))

print('Status:')
print(bots.values())
print(outputs.values())
print(outputs[0].numbers[0] * outputs[1].numbers[0] * outputs[2].numbers[0])
