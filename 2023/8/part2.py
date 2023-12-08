import math
from itertools import cycle

instructions = []
nodes = {}

for line in open("./input2"):
    if not instructions:
        instructions = [(0 if x == "L" else 1) for x in line.strip()]
        continue
    if line == "\n": continue
    name = line.split(" ")[0].strip()
    leftnode = line.split("(")[1].split(",")[0].strip()
    rightnode = line.split("(")[1].split(",")[1].split(")")[0].strip()
    nodes[name] = (leftnode, rightnode)

# count = 0
current = [(node, []) for node in nodes.keys() if node.endswith("A")]

for i, node in enumerate(current):
    instructionsc = cycle(instructions)
    instruction_offset = 0
    nnode = node[0]
    zets = []
    steps = 0
    while not nnode.endswith("Z"):
        nnode = nodes[nnode][next(instructionsc)]
        steps += 1
    # if not steps - 1 in zets:
    #     zets.append(steps - 1)
    current[i] = (node[0], steps)
    print(node[0], steps)

print(current)

values = [node[1] for node in current]

print(values)

# num = 0
# lowest_value = min(value[1] for value in values)
# print("low", lowest_value)

# while True:
#     num += lowest_value
#     # print(num)
#     if all([num % value[1] == value[0] for value in values]):
#         print("Found", num)
#         break

print(math.lcm(*[node for node in values]))