import fileinput
import string

total = 0
priorities = dict(pair[::-1] for pair in enumerate(string.ascii_letters, 1))
for line in fileinput.input():
    line = line.strip()
    l, r = line[len(line)//2:], line[:len(line)//2]
    common = set(l) & set(r)
    for letter in common:
        total += priorities[letter]
print(total)
