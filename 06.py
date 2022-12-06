import fileinput
from collections import deque

starts = []
buf = [line.strip() for line in fileinput.input()]


def find_start(msg, prefix_length):
    for line in msg:
        d = deque(line[:prefix_length], prefix_length)
        for i, x in enumerate(line[prefix_length:], prefix_length):
            if len(set(d)) == prefix_length:
                return i

            d.append(x)

        if len(set(d)) == prefix_length:
            return len(line) + 1

print(find_start(buf, 4))
print(find_start(buf, 14))
