copymap = [0] * 204

for num, line in enumerate(open("./input2")):
    matchingNums = len(([
        number for number in
        ((set(x.strip() for x in (line.split("|")[0].split(":")[1].split(" "))))
        & set(x.strip() for x in (line.split("|")[1].split(" "))))
        if number != ""
    ]))
    toAdd = copymap[num] + 1
    for i in range(1, matchingNums + 1):
        copymap[num + i] += toAdd

print(sum(copymap) + num + 1) # 6857330