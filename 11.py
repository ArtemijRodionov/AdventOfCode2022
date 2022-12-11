import fileinput
from collections import deque 


class Game:

    def __init__(self):
        self.monkeys = {}

    def join(self, monkey):
        self.monkeys[monkey.number] = monkey

    def throw(self, to, item):
        self.monkeys[to].catch(item)

    def __call__(self):
        for monkey in self.monkeys.values():
            for to, item in monkey.play():
                self.throw(to, item)

    def __repr__(self):
        return '<{}>'.format(',\n\t'.join(map(str, self.monkeys.values())))


class Operation:

    ops = {'+': lambda l, r: l + r, '*': lambda l, r: l * r}

    def __init__(self, op, lh, rh):
        self.op = op
        self.lh = lh
        self.rh = rh

    @classmethod
    def from_str(cls, xs):
        l, raw_op, r = xs.strip().split(' ')
        op = cls.ops[raw_op]

        def parse_operand(operand):
            handle = lambda x: x
            if operand != 'old':
                handle = lambda _: int(operand)
            return handle

        return cls(op, parse_operand(l), parse_operand(r))

    def __call__(self, old):
        return self.op(self.lh(old), self.rh(old))


class CountedOperation:

    def __init__(self, op):
        self.op = op
        self.counter = 0
    
    def __call__(self, old):
        self.counter += 1
        return self.op(old)


class Income:

    def __init__(self, test_value, true_income, false_income):
        self.test_value = test_value
        self.true = true_income
        self.false = false_income

    def __call__(self, value):
        if value % self.test_value == 0:
            return self.true
        return self.false


class Monkey:

    def __init__(self, number, op, income):
        self.number = number
        self.op = op
        self.income = income
        self.items = deque([])

    def catch(self, item):
        self.items.append(item)

    def play(self):
        while self.items:
            item = self.items.popleft()
            new_item = self.op(item) // 3
            throw_to = self.income(new_item)
            yield throw_to, new_item

    def __repr__(self):
        return f'<Monkey {self.number}: {list(self.items)}>'


def parse_monkey(xs):
    if (first_line := next(xs, None)) is None:
        return

    _, monkey_number = first_line.strip().split(' ')
    *_, starting_items = next(xs).strip().split(' ', 2)
    _, operation = next(xs).strip().split('= ', 1)
    *_, divisible_test = next(xs).strip().split(' ')
    *_, true_income = next(xs).strip().split(' ')
    *_, false_income = next(xs).strip().split(' ')
    # read empty line
    next(xs, None)

    monkey = Monkey(
        int(monkey_number.strip(':')),
        Operation.from_str(operation),
        Income(int(divisible_test), int(true_income), int(false_income)),
    )

    for item in starting_items.split(', '):
        monkey.catch(int(item))

    yield monkey

    yield from parse_monkey(xs)


def get_monkey_business(monkeys):
    *_, snd, fst = sorted([monkey.op.counter for monkey in monkeys])
    return snd * fst


monkeys = list(parse_monkey(fileinput.input()))
game = Game()
for monkey in monkeys:
    monkey.op = CountedOperation(monkey.op)
    game.join(monkey)

for _ in range(20):
    game()
print(get_monkey_business(monkeys))

