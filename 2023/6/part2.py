import math

time, distance = open("./input4")
time = time.split(":")[1]
distance = distance.split(":")[1]
time = [int(x) for x in time.split(" ") if x != ""]
distance = [int(x) for x in distance.split(" ") if x != ""]

out = []
for i, x in enumerate(time):
    dist = distance[i]
    count = 0
    for y in range(1, x):
        remaining = x - y
        traveled = y*remaining
        if traveled > dist:
            count += 1
    out.append(count)

print(out)
print(math.prod(out))