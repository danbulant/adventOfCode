
print(sum([2**(number-1) for number in [
    len(([
        number for number in
        (set(x.strip() for x in (line.split("|")[0].split(":")[1].split(" ")))) & set(x.strip() for x in (line.split("|")[1].split(" "))) if number != ""
    ]))
    for line in open("./input2")
] if number != 0]))