
previous = []
current = []

def finishmap():
    global previous
    global current
    print(current, previous)
    current += previous
    previous = current
    current = []

for line in open("input2"):
    if line.startswith("seeds:"):
        previous = [int(x) for x in line.split()[1:]]
        continue
    if line.endswith("map:\n"):
        print(line[:-5])
        finishmap()
        continue
    if len(line.split()) < 3: continue
    dest, source, length = line.split()
    dest = int(dest)
    source = int(source)
    length = int(length)
    print("Mapping {} to {} with length {}".format(source, dest, length))
    toRemove = []
    for i in previous:
        print(i, source, source + length)
        if i in range(source, source + length):
            current.append(dest + i - source)
            print("Changed {} to {}".format(i, dest + i - source))
            toRemove.append(i)
    for i in toRemove:
        previous.remove(i)

finishmap()
print(previous)
print(min(previous))