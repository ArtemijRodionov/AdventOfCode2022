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


def print_patch(patch, minx, maxx, miny, maxy):
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
    head, *body = rope
    for steps, direction in moves:
        for _ in range(steps):
            head += direction
            ahead = head
            for part in body:
                if part.distance(ahead) >= 2:
                    part += ahead.one(part)
                ahead = part
            yield rope

part1 = set(TailMove.from_vec2(tail) for _, tail in simulation([Vec2(), Vec2()]))
part2 = set(TailMove.from_vec2(tail) for *_, tail in simulation([Vec2() for _ in range(10)]))

print(len(part1))
print(len(part2))

import time
import os
if os.environ.get('PARTY'):
    for rope in simulation([Vec2() for _ in range(30)]):
        print_patch(list(map(TailMove.from_vec2, rope)), -30, 30, -30, 5)
        time.sleep(0.2)
        os.system('clear')

