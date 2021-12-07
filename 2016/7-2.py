import re
import sys

def match(a, b):
    return '{1}{0}{1}'.format(*a) == b

total = 0
for line in sys.stdin:
    babs = '-'.join(re.findall(r'\[([^]]+)\]', line))
    abas = re.sub(r'\[[^]]+\]', '-', line)
    print(line, babs, abas)
    m = re.findall(r'(?=([^-])((?!\1)[^-])(\1))', babs)
    bab = set([''.join(x) for x in m])
    print(bab)
    m = re.findall(r'(?=([^-])((?!\1)[^-])(\1))', abas)
    aba = set([''.join(x) for x in m])
    print(aba)
    #if bab.intersection(aba):
    if any(any(match(a, b) for b in bab) for a in aba):
    #if any(a in babs for a in aba) or any(b in abas for b in bab):
        total += 1
print(total)
