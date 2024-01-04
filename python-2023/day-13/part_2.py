import numpy as np

def is_off_by_one(a, b):
    off = 0
    for i in range(len(a)):
        if a[i] != b[i]:
            if off == 1:
                return False
            off = 1

    return off == 1

with open("input.txt") as data_file:
    patterns = [line.split("\n") for line in data_file.read().split("\n\n")]

    total = 0
    for pattern in patterns:
        grid = np.array([[r for r in p] for p in pattern])

        cols = []
        for i in range(0, grid.shape[1]-1):
            smudge = is_off_by_one(grid[:, i], grid[:, i+1])
            
            if list(grid[:, i]) == list(grid[:, i+1]) or smudge:
                c = 1
                valid = True

                while i-c != -1 and i+1+c != grid.shape[1] and valid:
                    if list(grid[:, i-c]) != list(grid[:, i+1+c]):
                        if smudge or not is_off_by_one(grid[:, i-c], grid[:, i+1+c]):
                            valid = False

                        if not smudge and is_off_by_one(grid[:, i-c], grid[:, i+1+c]):
                            valid = True 
                        
                        smudge = True
                    c += 1
                
                if valid:
                    cols_left= i+1
                    cols.append((i+1, smudge, 'c'))

        rows = []
        for i in range(0, len(grid)-1):
            smudge = is_off_by_one(grid[i, :], grid[i+1, :])

            if list(grid[i, :]) == list(grid[i+1, :]) or smudge:
                c = 1
                valid = True

                while i-c != -1 and i+1+c != len(grid) and valid:
                    if list(grid[i-c, :]) != list(grid[i+1+c, :]):
                        if smudge or not is_off_by_one(grid[i-c, :], grid[i+1+c, :]):
                            valid = False 

                        if not smudge and is_off_by_one(grid[i-c, :], grid[i+1+c, :]):
                            valid = True 
                        
                        smudge = True
                    c += 1
                
                if valid:
                    rows.append((i+1, smudge, 'r'))

        rows_top = 0
        cols_left = 0
        for i in rows + cols:
            if i[1]:
                if i[2] == 'r':
                    rows_top = i[0]
                if i[2] == 'c':
                    cols_left = i[0]

        total += cols_left + rows_top * 100

    print(total)
