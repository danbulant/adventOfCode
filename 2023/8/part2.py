import math
from itertools import cycle

instructions = []
nodes = {}

for line in open("./input2"):
    if not instructions:
        instructions = [(0 if x == "L" else 1) for x in line.strip()]
        continue
    if line == "\n": continue
    nodes[line.split(" ")[0].strip()] = line.split("(")[1].split(")")[0].split(", ")

def node_value(node):
    instructionsc = cycle(instructions)
    steps = 0
    while not node.endswith("Z"):
        node = nodes[node][next(instructionsc)]
        steps += 1
    return steps

current = [node_value(node) for node in nodes.keys() if node.endswith("A")]

print(math.lcm(*current))