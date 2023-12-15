from functools import lru_cache

map: list[list[str]] = []

for line in open("./input1"):
    map.append([x for x in line.strip()])


def printmap():
    for line in map:
        print("".join(line))

printmap()

# @lru_cache(maxsize=20)
def north(map):
    moved = 1
    while moved:
        moved = 0
        for i in range(1, len(map)):
            for j in range(len(map[i])):
                if map[i][j] == "O" and map[i-1][j] == ".":
                    map[i][j] = "."
                    map[i-1][j] = "O"
                    moved += 1

# @lru_cache(maxsize=20)
def west(map):
    moved = 1
    while moved:
        moved = 0
        for i in range(len(map)):
            for j in range(1, len(map[i])):
                if map[i][j] == "O" and map[i][j-1] == ".":
                    map[i][j] = "."
                    map[i][j-1] = "O"
                    moved += 1
# @lru_cache(maxsize=20)
def south(map):
    moved = 1
    while moved:
        moved = 0
        for i in range(len(map)-1):
            for j in range(len(map[i])):
                if map[i][j] == "O" and map[i+1][j] == ".":
                    map[i][j] = "."
                    map[i+1][j] = "O"
                    moved += 1
# @lru_cache(maxsize=20)
def east(map):
    moved = 1
    while moved:
        moved = 0
        for i in range(len(map)):
            for j in range(len(map[i])-1):
                if map[i][j] == "O" and map[i][j+1] == ".":
                    map[i][j] = "."
                    map[i][j+1] = "O"
                    moved += 1

def cycle():
    north(map)
    west(map)
    south(map)
    east(map)

for i in range(1_000_000_000):
    cycle()

print()

printmap()



value = 0

for i in range(len(map)):
    for j in range(len(map[i])):
        if map[i][j] == "O":
            value += len(map) - i

print(value)