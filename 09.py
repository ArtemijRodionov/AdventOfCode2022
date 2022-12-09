import math
import fileinput
from dataclasses import dataclass


def norm(v):
    return int(math.copysign(1.0, v))


@dataclass(frozen=True, slots=True)
class Vec2:
    x: int = 0
    y: int = 0

    def one(self):
        return Vec2(norm(self.x), norm(self.y))

    def length(self):
        return math.sqrt(self.x**2 + self.y**2)

    def distance(self, to):
        return (self - to).length()

    def __sub__(self, vec2):
        return Vec2(self.x - vec2.x, self.y - vec2.y)

    def __add__(self, vec2):
        return Vec2(self.x + vec2.x, self.y + vec2.y)

def print_patch(patch):
    minx = min(patch, key=lambda x: x.x).x - 1
    miny = min(patch, key=lambda x: x.y).y - 1
    maxx = max(patch, key=lambda x: x.x).x + 1 
    maxy = max(patch, key=lambda x: x.y).y + 1
    print(sorted(patch, key=lambda x: (x.y, x.x)))

    cols = []
    for y in reversed(range(miny, maxy)): 
        row = []
        for x in range(minx, maxx):
            if Vec2(x, y) in patch:
                row.append('#')
            else:
                row.append('.')
        cols.append(''.join(row))
    print('\n'.join(cols))

dirs = {'L': Vec2(-1, 0), 'R': Vec2(1, 0), 'U': Vec2(0, 1), 'D': Vec2(0, -1)}
moves = []

for line in fileinput.input():
    direction, steps = line.strip().split(' ')
    moves.append((int(steps), dirs[direction]))

head = Vec2()
tail = Vec2()
patch = set()
for steps, direction in moves:
    for _ in range(steps):
        head += direction
        if tail.distance(head) < 2:
            continue
        if head.x == tail.x or head.y == tail.y:
            tail += direction
        else:
            tail += (head - tail).one()
        patch.add(tail)

print(len(patch) + 1)
#print_patch(patch)
