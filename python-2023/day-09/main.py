with open("input.txt") as data_file:
    nums = [[int(n) for n in line.split(' ')] for line in data_file.read().split("\n") if line != ""]

    result = 0
    
    idx = 0
    while idx != len(nums):
        history = nums[idx]
        steps = [history]
        i = 0

        while not (len(set(history)) == 1 and list(set(history))[0] == 0):
            diffs = [history[j+1] - n for j, n in enumerate(history) if j < len(history)-1]
            steps.append(diffs)
            history = diffs
            i += 1

        steps.reverse()

        for i, step in enumerate(steps):
            if i+1 == len(steps):
                break

            steps[i+1].append(step[-1] + steps[i+1][-1])

        result += steps[-1][-1]
        idx += 1

    print(result)

# part 2
with open("input.txt") as data_file:
    nums = [[int(n) for n in line.split(' ')] for line in data_file.read().split("\n") if line != ""]
    for history in nums:
        history.reverse()

    result = 0
    
    idx = 0
    while idx != len(nums):
        history = nums[idx]
        steps = [history]
        i = 0


        while not (len(set(history)) == 1 and list(set(history))[0] == 0):
            diffs = [(history[j+1] - n) * -1 for j, n in enumerate(history) if j < len(history)-1]
            steps.append(diffs)
            history = diffs
            i += 1

        steps.reverse()

        for i, step in enumerate(steps):
            if i+1 == len(steps):
                break
            
            steps[i+1].append(steps[i+1][-1] - step[-1])


        result += steps[-1][-1]
        idx += 1

    print(result)