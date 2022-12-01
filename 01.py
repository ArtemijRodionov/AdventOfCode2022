import fileinput
import heapq as hq

max_heap = []
value = 0
for line in fileinput.input(encoding='utf-8'):
    if not (line := line.strip()):
        if len(max_heap) == 3:
            hq.heappushpop(max_heap, value)
        else:
            hq.heappush(max_heap, value)
        value = 0
        continue 
    value += int(line)
 
print(max(max_heap))
print(sum(max_heap))

