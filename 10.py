import fileinput


class CPU:
    def __init__(self):
        self.cycles = 0
        self.x = 1

    def __call__(self, op):
        op(self)
        self.cycles += op.cycles


class Noop:
    cycles = 1

    def __call__(self, cpu):
        pass


class AddX:
    cycles = 2

    def __init__(self, n):
        self.n = n

    def __call__(self, cpu):
        cpu.x += self.n


class Marker:
    def __init__(self, cpu):
        self.cpu = cpu
        self.signals = []

        self._marks = [220, 180, 140, 100, 60, 20]
        self._mark = self._marks.pop()

    def __call__(self, op):
        x = self.cpu.x
        self.cpu(op)
        if self._mark and self.cpu.cycles >= self._mark:
            self.signals.append(x * self._mark)
            self._mark = self._marks.pop() if self._marks else None


class CRT:
    def __init__(self, cpu):
        self.cycles = cpu.cycles
        self.x = cpu.x
        self.segments = [['.'] * 40 for _ in range(6)]

    def __call__(self, cpu):
        x = self.x - 1
        while self.cycles < cpu.cycles:
            segment = self.segments[self.cycles // 40]
            segment_i = self.cycles % 40
            if x <= segment_i <= x + 2:
                segment[segment_i] = '#'
            self.cycles += 1
        self.x = cpu.x

    def print(self):
        print('\n'.join([''.join(segment) for segment in self.segments]))
        
ops = [] 
for line in fileinput.input():
    line = line.strip()
    if line == 'noop':
        ops.append(Noop())
    elif line.startswith('addx'):
        _, n = line.split(' ')
        ops.append(AddX(int(n)))
    else:
        raise Exception()

cpu = CPU()
marker = Marker(cpu)
crt = CRT(cpu)
for op in ops:
    marker(op)
    crt(cpu)

signals = marker.signals
print(signals)
print(sum(signals))
crt.print()

