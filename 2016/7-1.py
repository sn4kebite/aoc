import re
import sys

total = 0
for line in sys.stdin:
    m = re.search(r'\[[^]]*([^]])((?!\1|]).)(\2\1)', line)
    if m:
        continue
    m = re.search(r'(.)((?!\1).)(\2\1)', line)
    if m and m.group(1) != m.group(2):
        total += 1
print(total)
