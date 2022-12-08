import fileinput
import numpy as np

grid = np.array([list(map(int, l.strip())) for l in fileinput.input()])
x, y = grid.shape
is_visible = lambda xs, val: xs.size == 0 or max(xs) < val
visible_count = 0
best_scenic_score = 0
for i in range(x):
    for j in range(y):
        height = grid[i,j]
        sides = [grid[:i,j][::-1], grid[i+1:,j], grid[i,j+1:], grid[i,:j][::-1]]
        if any(is_visible(side, height) for side in sides):
            visible_count += 1
        scenic_score = 1
        for side in sides:
            visible_trees = 0
            for side_height in side:
                visible_trees += 1
                if side_height >= height:
                    break
            scenic_score *= visible_trees
        best_scenic_score = max(scenic_score, best_scenic_score)

print(visible_count)
print(best_scenic_score)

