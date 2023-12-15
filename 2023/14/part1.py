
map: list[list[str]] = []

for line in open("./input2"):
    map.append([x for x in line.strip()])


def printmap():
    for line in map:
        print("".join(line))

printmap()

moved = 1
while moved:
    moved = 0
    for i in range(1, len(map)):
        for j in range(len(map[i])):
            if map[i][j] == "O" and map[i-1][j] == ".":
                map[i][j] = "."
                map[i-1][j] = "O"
                moved += 1

print()

printmap()

value = 0

for i in range(len(map)):
    for j in range(len(map[i])):
        if map[i][j] == "O":
            value += len(map) - i

print(value)