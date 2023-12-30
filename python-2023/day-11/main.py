import numpy as np

def get_sum_of_lengths(gap):
    result = 0
    for pair in pairs:
        y1 = pair[0][0]
        y2 = pair[1][0]
        x1 = pair[0][1]
        x2 = pair[1][1]

        col_diff = 0
        for c in range(min(x1, x2), max(x1, x2)):
            if c in cols:
                col_diff += gap - 1

        row_diff = 0
        for r in range(min(y1, y2), max(y1, y2)):
            if r in rows:
                row_diff += gap - 1

        result += abs(x1 - x2 ) + abs(y1 - y2 ) + row_diff + col_diff

    return result


with open("input.txt") as data_file:
    grid = [[i for i in line] for line in data_file.read().split("\n") if line != ""]

    rows = []
    for i, row in enumerate(grid):
        if len(set(row)) == 1 and list(set(row))[0] == ".":
            rows.append(i)

    grid = np.array(grid)
    cols = []
    for i, col in enumerate(range(grid.shape[1])):
        if len(set(grid[:, col])) == 1 and list(set(grid[:, col]))[0] == ".":
            cols.append(i)

    row_indices, col_indices = np.where(grid == "#")
    coordinates = list(zip(row_indices, col_indices))
    
    pairs = set()
    for i in range(len(coordinates)):
        for j in range(i + 1, len(coordinates)):
            pair = (coordinates[i], coordinates[j])
            pairs.add(pair)

    print(get_sum_of_lengths(2))
    print(get_sum_of_lengths(1_000_000))
