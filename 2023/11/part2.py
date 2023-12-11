
map = []

for line in open("./input2"):
    map.append([x for x in line.strip()])

multiplier = 1_000_000 -1

emptyRows = []
emptyCols = []

for i in range(len(map)):
    if map[i].count(".") == len(map[i]):
        emptyRows.append(i)

for i in range(len(map[0])):
    if [x[i] for x in map].count(".") == len(map):
        emptyCols.append(i)

emptyRows.reverse()
emptyCols.reverse()

# for line in map:
#     print("".join(line))

print(emptyRows)
print(emptyCols)

galaxies = []

for i in range(len(map)):
    for j in range(len(map[i])):
        if map[i][j] == "#":
            galaxies.append((i + (len([x for x in emptyRows if x < i])) * multiplier, j + (len([x for x in emptyCols if x < j])) * multiplier))

counter = 0
for x in range(len(galaxies)):
    for y in range(x, len(galaxies)):
        if x == y: continue
        galaxy = galaxies[x]
        galaxy2 = galaxies[y]
        counter += abs(galaxy[0] - galaxy2[0]) + abs(galaxy[1] - galaxy2[1])

print(counter)