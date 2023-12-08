
instructions = []
nodes = {}

for line in open("./input2"):
    if not instructions:
        instructions = [x for x in line.strip()]
        continue
    if line == "\n": continue
    name = line.split(" ")[0].strip()
    leftnode = line.split("(")[1].split(",")[0].strip()
    rightnode = line.split("(")[1].split(",")[1].split(")")[0].strip()
    nodes[name] = (leftnode, rightnode)

node = "AAA"
count = 0

while node != "ZZZ":
    node = nodes[node][1 if instructions[0] == "R" else 0]
    instructions = instructions[1:] + instructions[:1]
    count += 1

print(count)