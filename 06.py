import fileinput
from collections import deque

starts = []
for line in fileinput.input():
    d = deque(line[:4])
    for i, x in enumerate(line[4:], 4):
        if len(set(d)) == 4:
            starts.append(i)
            break

        d.append(x)
        d.popleft()

    if not starts and len(set(d)) == 4:
        starts.append(len(line)+1)

print(starts)
