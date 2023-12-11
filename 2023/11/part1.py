
map = []

for line in open("./input1"):
    map.append([x for x in line.strip()])

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

for i in emptyRows:
    # Add another empty row at the selected index
    map.insert(i, ["." for x in range(len(map[0]))])

for i in emptyCols:
    # Add another empty column at the selected index
    for j in range(len(map)):
        map[j].insert(i, ".")

# for line in map:
#     print("".join(line))

galaxies = []

for i in range(len(map)):
    for j in range(len(map[i])):
        if map[i][j] == "#":
            galaxies.append((i, j))

counter = 0
for x in range(len(galaxies)):
    for y in range(x, len(galaxies)):
        if x == y: continue
        galaxy = galaxies[x]
        galaxy2 = galaxies[y]
        counter += abs(galaxy[0] - galaxy2[0]) + abs(galaxy[1] - galaxy2[1])

print(counter)