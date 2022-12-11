import fileinput

full_overlap = 0
overlap = 0
for line in fileinput.input():
    l, r = line.split(',', 1)
    split_interval = lambda x: tuple(map(int, x.split('-', 1)))
    (ll, lr), (rl, rr) = split_interval(l), split_interval(r)
    if (ll <= rl and lr >= rr) or (ll >= rl and lr <= rr):
        full_overlap += 1
        overlap += 1
    elif max(ll, rl) <= min(lr, rr):
        overlap += 1

print(full_overlap)
print(overlap)
