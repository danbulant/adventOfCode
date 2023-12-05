previous: list[range] = []
current: list[range] = []

def finishmap():
    global previous
    global current
    print(current, previous)
    current += previous
    previous = current
    current = []
    previous = mergeRanges(previous)

def mergeRanges(ranges: list[range]) -> list[range]:
    ranges.sort(key=lambda x: x.start)
    newRanges: list[range] = []
    for i in ranges:
        if len(newRanges) == 0:
            newRanges.append(i)
            continue
        if not overlap(newRanges[-1], i):
            newRanges.append(i)
            continue
        newRanges[-1] = range(min(newRanges[-1].start, i.start), max(newRanges[-1].stop, i.stop))
        newRanges.append(i)
    return newRanges

def overlap(range1: range, range2: range):
    if range1.start >= range2.stop or range1.stop <= range2.start:
        return False
    return True

def getOverlapRange(range1: range, range2: range) -> tuple[range|None,range,range|None]:
    if not overlap(range1, range2):
        raise Exception("Ranges do not overlap")
    return (
        range(range1.start, range2.start) if range1.start < range2.start else None,
        range(max(range1.start, range2.start), min(range1.stop, range2.stop)),
        range(range2.stop, range1.stop) if range1.stop > range2.stop else None
    )

def splitRange(range1: range, at: int) -> tuple[range,range]:
    if not at in range1:
        raise Exception("at not in range")
    return (range(range1.start, at), range(at, range1.stop))

for line in open("input2"):
    if line.startswith("seeds:"):
        split = line.split()[1:]
        previous = [range(int(x), int(x) + int(y)) for x, y in zip(split[::2], split[1::2])]
        continue
    if line.endswith("map:\n"):
        print(line[:-5])
        finishmap()
        continue
    if len(line.split()) < 3: continue
    dest, source, length = line.split()
    dest = int(dest)
    source = int(source)
    offset = dest - source
    length = int(length)
    print("Mapping {} to {} with length {}".format(source, dest, length))
    sourceRange = range(source, source + length)
    splitSomething = True
    maxloops = 10
    while splitSomething:
        toRemove: list[range] = []
        splitSomething = False
        newPrevious: list[range] = []
        for i in previous:
            if not overlap(i, sourceRange): continue
            splitSomething = True
            toRemove.append(i)
            if i.start == sourceRange.start and i.stop == sourceRange.stop:
                current.append(range(dest, dest + length))
                print("Changed1 {} to {}".format(i, range(dest, dest + length)))
                continue
            nonoverlap1, overlapping, nonoverlap2 = getOverlapRange(i, sourceRange)
            if nonoverlap1 is not None and nonoverlap1.start < nonoverlap1.stop:
                newPrevious.append(nonoverlap1)
            current.append(range(overlapping.start + offset, overlapping.stop + offset))
            if nonoverlap2 is not None and nonoverlap2.start < nonoverlap2.stop:
                newPrevious.append(nonoverlap2)
        for i in toRemove:
            previous.remove(i)
        previous += newPrevious
        previous = mergeRanges(previous)
        print("p", previous, current)

finishmap()
print(previous)
print(min(prev.start for prev in previous))
