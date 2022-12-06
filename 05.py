import fileinput
import re

crates_re = re.compile('\[([A-Z])\]|( {4})')
ops_re = re.compile('move (\d+) from (\d+) to (\d+)')

is_ops = False
stacks = []
ops = []
for line in fileinput.input():
    if not line.strip():
        continue
    if is_ops:
        (op, ) = ops_re.findall(line)
        ops.append(tuple(map(int, op)))
    else:
        crate_level = crates_re.findall(line)
        if len(crate_level) == 0:
            is_ops = True
        assert len(crate_level) <= 9
        for i_stack, (crate_name, _) in enumerate(crate_level):
            if i_stack == len(stacks):
                stacks.append([])
            if crate_name:
                stacks[i_stack].append(crate_name)

stacks_9001 = []
for i, stack in enumerate(stacks):
    stacks[i].reverse()
    stacks_9001.append(stack[:])

for count, from_, to_ in ops:
    from_stack = stacks[from_ - 1]
    to_stack = stacks[to_ - 1]
    for _ in range(count):
        to_stack.append(from_stack.pop())

for count, from_, to_ in ops:
    from_stack = stacks_9001[from_ - 1]
    to_stack = stacks_9001[to_ - 1]
    buffer = []
    for _ in range(count):
        buffer.append(from_stack.pop())
    while buffer:
        to_stack.append(buffer.pop())

print(''.join([stack[-1] for stack in stacks if stack]))
print(''.join([stack[-1] for stack in stacks_9001 if stack]))

