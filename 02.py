import fileinput

move_scores = {'X': 1, 'Y': 2, 'Z': 3}
incomes = {
    # rock, paper, scissors
    zip('ABC', 'ZXY'): 0,
    zip('ABC', 'XYZ'): 3,
    zip('ABC', 'YZX'): 6,
}
income_scores = {}
for options, score in incomes.items():
    for option in options:
        income_scores[option] = score
total = 0
for line in fileinput.input():
    abc, xyz = move = line.strip().split(' ', 1)
    move_score = move_scores[xyz]
    income_score = income_scores[tuple(move)]
    total += move_score + income_score

print(total)

