import fileinput

count = 0
for line in fileinput.input():
    l, r = line.split(',', 1)
    split_interval = lambda x: tuple(map(int, x.split('-', 1)))
    (ll, lr), (rl, rr) = split_interval(l), split_interval(r)
    if (ll <= rl and lr >= rr) or (ll >= rl and lr <= rr):
        count += 1

print(count)
