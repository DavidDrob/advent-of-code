categoryMap = ["x", "m", "a", "s"]

with open("input.txt") as data_file:
    lines = data_file.read().split("\n")
    split = lines.index('')

    workflows_data = lines[:split]
    parts = lines[split+1:]

    workflows = {}

    for workflow in workflows_data:
        workflow = workflow[0: -1].split("{")

        name = workflow[0]
        rules = [rule.split(":") for rule in workflow[1].split(",")]

        workflows[name] = rules


    def get_next(name):
        rules = workflows[name]

        for rule in rules:

            if len(rule) > 1:
                if "<" in rule[0]:
                    (category, right_value) = rule[0].split("<")
                    left_value = part[categoryMap.index(category)]

                    if int(left_value) < int(right_value):
                        if len(rule[1]) == 1:
                            if rule[1] == "A":
                                valid.append(P_IDX)
                            break
                        else:
                            get_next(rule[1])
                            break


                elif ">" in rule[0]:
                    (category, right_value) = rule[0].split(">")
                    left_value = part[categoryMap.index(category)]

                    if int(left_value) > int(right_value):
                        if len(rule[1]) == 1:
                            if rule[1] == "A":
                                valid.append(P_IDX)
                            break
                        else:
                            get_next(rule[1])
                            break

            else:
                if len(rule[0]) == 1:
                    if rule[0] == "A":
                        valid.append(P_IDX)
                    break
                else:
                    get_next(rule[0])

    valid = []


    for i in range(len(parts) - 1):
        P_IDX = i
        part = [p.split("=")[1] for p in parts[P_IDX][1:-1].split(",")]

        get_next("in")


    total = 0

    for valid_part in valid:
        total += sum([int(p.split("=")[1]) for p in parts[valid_part][1:-1].split(",")])

    print(total)
