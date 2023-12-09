def get_differences(numbers: list[int]) -> list[int]:
    differences = []
    for i in range(1, len(numbers)):
        differences.append(numbers[i] - numbers[i - 1])
    return differences

values = []

for line in open("./input2"):
    history = [[int(x) for x in line.split(" ")]]
    while not all(x == 0 for x in history[-1]):
        history.append(get_differences(history[-1]))
    history.reverse()
    for i in range(len(history)):
        if i == 0:
            history[i].insert(0, 0)
            continue
        history[i].insert(0, history[i][0] - history[i - 1][0])
    print(history)
    values.append(history[-1][0])

print(values)
print(sum(values))