import sys
sys.setrecursionlimit(3000)

with open("input.txt") as data_file:
    grid = [[i for i in line] for line in data_file.read().split("\n") if line != ""]
    COLS = len(grid) - 1
    ROWS = len(grid[0]) - 1
    
    pos = (0, 0) ## y, x
    direction = 'r'
    visited = {pos}
    visited_pipes = []

    changes = {
        ('r', '|'): ['u', 'd'],
        ('l', '|'): ['u', 'd'],
        ('d', '|'): ['d'],
        ('d', '.'): ['d'],
        ('d', '/'): ['l'],
        ('d', '-'): ['l', 'r'],
        ('u', '-'): ['l', 'r'],
        ('l', '.'): ['l'],
        ('r', '.'): ['r'],
        ('r', '-'): ['r'],
        ('l', '-'): ['l'],
        ('l', '/'): ['d'],
        ('r', '/'): ['u'],
        ('u', '/'): ['r'],
        ('u', '|'): ['u'],
        ('u', '.'): ['u'],
        ('u', '\\'): ['l'],
        ('r', '\\'): ['d'],
        ('l', '\\'): ['u'],
        ('d', '\\'): ['r'],
    }

    get_next = {
        'r': [0, 1],
        'd': [1, 0],
        'l': [0, -1],
        'u': [-1, 0],
    }

    def go(pos, dir):
        coords = get_next[dir]
        new_pos = (pos[0] + coords[0], pos[1] + coords[1])

        if new_pos[0] <= COLS and new_pos[0] >= 0 and new_pos[1] <= ROWS and new_pos[1] >= 0:
            visited.add(new_pos)
            next_char = grid[new_pos[0]][new_pos[1]]

            current_data = (new_pos, next_char, dir)
            directions = changes[(dir, next_char)]

            if (current_data not in visited_pipes):
                visited_pipes.append(current_data)
                for i in directions:
                    go(new_pos, i)

    directions = changes[(direction, grid[pos[0]][pos[1]])]
    for i in directions:
        go(pos, i)

    print(len(visited))