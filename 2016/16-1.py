import sys

data = sys.stdin.read().rstrip()
data_length = int(sys.argv[1])

while len(data) < data_length:
    data = '{}0{}'.format(data, ''.join('1' if x == '0' else '0' for x in data[::-1]))
    print(data)

checksum = ''
while len(checksum) % 2 == 0:
    checksum = ''
    for pair in zip(data[:data_length:2], data[1:data_length:2]):
        checksum += '1' if pair[0] == pair[1] else '0'
    data = checksum
print(checksum)
