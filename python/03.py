import fileinput
import string

total = 0
priorities = dict(pair[::-1] for pair in enumerate(string.ascii_letters, 1))
groups = [[]]
for line in fileinput.input():
    line = line.strip()
    l, r = line[len(line)//2:], line[:len(line)//2]
    common = set(l) & set(r)
    for letter in common:
        total += priorities[letter]
    groups[-1].append(line)
    if len(groups[-1]) == 3:
        groups.append([])
print(total)

total = 0
groups = list(filter(None, groups))
for group in groups:
    common = set(group[0])
    for line in group[1:]:
        common &= set(line)
    for letter in common:
        total += priorities[letter]
print(total)

