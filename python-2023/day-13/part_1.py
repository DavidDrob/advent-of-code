import numpy as np

with open("input.txt") as data_file:
    patterns = [line.split("\n") for line in data_file.read().split("\n\n")]

    total = 0
    for pattern in patterns:
        grid = np.array([[r for r in p] for p in pattern])

        cols_left = 0

        for i in range(0, grid.shape[1]-1):
            if list(grid[:, i]) == list(grid[:, i+1]):
                c = 1
                valid = True

                while i-c != -1 and i+1+c != grid.shape[1] and valid:
                    if list(grid[:, i-c]) != list(grid[:, i+1+c]):
                        valid = False
                    c += 1
                
                if valid:
                    cols_left = i+1


        rows_top = 0

        for i in range(0, len(grid)-1):
            if list(grid[i, :]) == list(grid[i+1, :]):
                c = 1
                valid = True

                while i-c != -1 and i+1+c != len(grid) and valid:
                    if list(grid[i-c, :]) != list(grid[i+1+c, :]):
                        valid = False
                    c += 1
                
                if valid:
                    rows_top = i+1


        total += cols_left + rows_top * 100

    print(total)