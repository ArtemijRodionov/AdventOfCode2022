import fileinput

max_value = 0
value = 0
for line in fileinput.input(encoding='utf-8'):
    line = line.strip()
    if not line:
        max_value = max(max_value, value)
        value = 0
        continue 
    value += int(line)
 
print(max_value)

