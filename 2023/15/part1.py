
content = ""

for line in open("./input2"):
    content += line.strip()

values = []
for part in line.split(","):
    currentValue = 0
    for letter in part:
        asciiValue = ord(letter)
        currentValue += asciiValue
        currentValue *= 17
        currentValue %= 256

    values.append(currentValue)

print(values)
print(sum(values))