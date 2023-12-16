with open("input.txt") as data_file:
    grid = [[i for i in line] for line in data_file.read().split("\n") if line != ""]
    
    pos = ()
    for i in range(len(grid)):
        for j in range(len(grid)):
            if grid[i][j] == "S":
                pos = (i, j)
                
    pipes = {
        '-': [(0, 1), (0, -1)],
        '|': [(1, 0), (-1, 0)],
        '7': [(1, 0), (0, -1)],
        'L': [(-1, 0), (0, 1)],
        'J': [(0, -1), (-1, 0)],
        'F': [(0, 1), (1, 0)],
        'S': [(0, 0), (0, 0)]
    }

    last = pos

    y = pos[0]
    x = pos[1]
    if grid[y-1][x][0] in [key for key in pipes]:
        pos = (y-1, x)
    elif grid[y+1][x][0] in [key for key in pipes]:
        pos = (y+1, x)
    elif grid[y][x-1][0] in [key for key in pipes]:
        pos = (y, x-1)
    elif grid[y][x+1][0] in [key for key in pipes]:
        pos = (y, x+1)


    prev = pos
    idx = 0
    while True:
        y = pos[0]
        x = pos[1]

        if grid[y][x] == "S":
            break

        is_dot = False 
        if grid[y][x] == '.':
            is_dot = True
        else: 
            newY = pos[0] + pipes[grid[y][x]][0][0]
            newX = pos[1] + pipes[grid[y][x]][0][1]

        direction = 0
        if (newY, newX) == last or is_dot:
            direction = 1

        last = pos

        pos = list(pos)
        pos[0] += pipes[grid[y][x]][direction][0]
        pos[1] += pipes[grid[y][x]][direction][1]
        pos = tuple(pos)

        idx += 1

    print(int((idx + 1) / 2))