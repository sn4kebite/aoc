import sys

from pylab import *

grid(1)
for filename in sys.argv[1:]:
    print('plotting', filename)
    with open(filename, 'rt') as f:
        data = []
        for line in f:
            line = line.strip()
            if not line or not line.isdigit():
                continue
            data.append(int(line))
        plot(range(len(data)), data)
legend(sys.argv[1:])

show()
