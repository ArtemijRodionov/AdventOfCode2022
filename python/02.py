import fileinput

# rock, paper, scissors
move_scores = {'X': 1, 'Y': 2, 'Z': 3}
lose = tuple(zip('ABC', 'ZXY'))
draw = tuple(zip('ABC', 'XYZ'))
win = tuple(zip('ABC', 'YZX'))
incomes = {
    lose: 0,
    draw: 3,
    win: 6,
}
rules = {
    # lose
    'X': dict(lose),
    # draw
    'Y': dict(draw),
    # win
    'Z': dict(win),
}
income_scores = {}
for options, score in incomes.items():
    for option in options:
        income_scores[option] = score

total_first = 0
total_second = 0
for line in fileinput.input():
    abc, xyz = move = line.strip().split(' ', 1)
    move_score = move_scores[xyz]
    income_score = income_scores[tuple(move)]
    total_first += move_score + income_score

    ruled_xyz = rules[xyz][abc]
    ruled_move_score = move_scores[ruled_xyz]
    ruled_income_score = income_scores[(abc, ruled_xyz)]
    total_second += ruled_move_score + ruled_income_score

print(total_first)
print(total_second)

