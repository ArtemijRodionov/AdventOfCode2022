import math
import fileinput
from dataclasses import dataclass


def norm(v):
    return 0 if v == 0 else int(math.copysign(1.0, v))


@dataclass(slots=True)
class Vec2:
    x: int = 0
    y: int = 0

    def length(self):
        return math.sqrt(self.x**2 + self.y**2)

    def one(self, to):
        n = self - to
        return Vec2(norm(n.x), norm(n.y))

    def distance(self, to):
        return (self - to).length()

    def __sub__(self, vec2):
        return Vec2(self.x - vec2.x, self.y - vec2.y)

    def __isub__(self, vec2):
        self.x -= vec2.x
        self.y -= vec2.y
        return self

    def __add__(self, vec2):
        return Vec2(self.x + vec2.x, self.y + vec2.y)

    def __iadd__(self, vec2):
        self.x += vec2.x
        self.y += vec2.y
        return self


@dataclass(frozen=True, slots=True)
class TailMove:
    x: int = 0
    y: int = 0

    @classmethod
    def from_vec2(cls, vec2):
        return cls(vec2.x, vec2.y)


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
            if TailMove(x, y) in patch:
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


def simulation(rope):
    tail_patch = set()
    head, *body = rope
    for steps, direction in moves:
        for _ in range(steps):
            head += direction
            ahead = head
            for part in body:
                if part.distance(ahead) >= 2:
                    part += ahead.one(part)
                ahead = part
            tail_patch.add(TailMove.from_vec2(body[-1]))
    return tail_patch

part1 = simulation([Vec2(), Vec2()])
part2 = simulation([Vec2() for _ in range(10)])
print(len(part1))
#print_patch(part2)
print(len(part2))
