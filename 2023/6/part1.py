import math

time, distance = open("./input2")
distance = [int(x) for x in distance.split(":")[1].split(" ") if x != ""]

print(math.prod([
    len(
        [y for y in range(1, x) if y*(x-y) > distance[i]]
    )
    for i, x in enumerate([int(x) for x in time.split(":")[1].split(" ") if x != ""])
]))
