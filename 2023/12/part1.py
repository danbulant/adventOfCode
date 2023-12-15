
def find_groups(str: str):
    groups = []
    cur = 0
    for c in str:
        if c == "#":
            cur += 1
        if c == ".":
            if cur > 0:
                groups.append(cur)
                cur = 0
    if cur > 0:
        groups.append(cur)
    return groups

def gen_parts(string: str):
    max_num = 2 ** string.count("?")
    for i in range(max_num):
        substr = string
        # replace each ? with a 0 or 1 based on i
        for j in range(string.count("?")):
            substr = substr.replace("?", str("." if (i >> j & 1) else "#"), 1)
        yield substr


count = 0

for line in open("./input2"):
    chars = line.split(" ")[0]
    groups = [int(x) for x in line.split(" ")[1].split(",")]

    print(line)

    for part in gen_parts(chars):
        found = find_groups(part)
        # print("part", part, found)
        if found == groups:
            # print("found", part)
            count += 1

print(count)