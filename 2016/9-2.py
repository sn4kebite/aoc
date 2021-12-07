import re
import sys

def decompress(buf):
    global compressed
    print('buf={}'.format(buf))
    m = re.match(r'\((\d+)x(\d+)\)', buf)
    length = int(m.group(1))
    count = int(m.group(2))
    start = m.span()[1]
    buf = buf[start:start+length]
    print(buf[:start+length], 'length={} count={}, start={}'.format(length, count, start))
    #compressed = compressed[start:]
    remaining = length
    current_total = 0
    while len(buf):
        #print('remaining={}, compressed={}'.format(remaining, compressed[offset:offset + 10]))
        #if compressed[offset + start] == '(':
        if buf[0] == '(':
            decompressed_length, decompressed_count, size, skip = decompress(buf)
            #offset += decompressed_length
            #size = decompressed_length * decompressed_count * count
            buf = buf[skip+decompressed_length:]
            print('size={} length={} count={} total={}'.format(size, length, count, size * count))
            current_total += size * count
            remaining -= decompressed_length * decompressed_count
        else:
            size = length * count
            #print('{}: length={} count={} size={}'.format(compressed[offset + start:offset + start +length], length, count, size))
            print('{}: length={} count={} size={}'.format(buf[:length], length, count, size))
            #compressed = compressed[offset+length:]
            buf = buf[length:]
            current_total += size
            #offset += length
            remaining -= length
    return length, count, current_total, start

compressed = sys.stdin.read().rstrip()
total = 0
while len(compressed):
    m = re.match(r'^(\w+)', compressed)
    if m:
        length = m.span()[1]
        total += length
        print('{}: length={}'.format(compressed[:length], length))
        compressed = compressed[length:]
    elif compressed[0] == '(':
        length, count, decompressed_size, start = decompress(compressed)
        #decompressed_size = length * count
        print('decompressed_size={}'.format(decompressed_size))
        total += decompressed_size
        compressed = compressed[length+start:]
    else:
        raise RuntimeError('fasdf')
print(total)
