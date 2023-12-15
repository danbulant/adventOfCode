import re

content = ""

for line in open("./input2"):
    content += line.strip()

boxes = []
for i in range(256):
    boxes.append([])
# print(boxes)
for part in line.split(","):
    currentValue = 0
    key, value = re.split(r"=|-", part)
    for letter in part:
        if letter == "=":
            # find if key is in boxes[currentValue]. Boxes is a list of tuples with (key, value)
            # if it is, replace it
            # if it is not, append it
            for i in range(len(boxes[currentValue])):
                if boxes[currentValue][i][0] == key:
                    boxes[currentValue][i] = (key, value)
                    break
            else:
                boxes[currentValue].append((key, value))
            break
        if letter == "-":
            boxes[currentValue] = [x for x in boxes[currentValue] if x[0] != key]
            break
        asciiValue = ord(letter)
        currentValue += asciiValue
        currentValue *= 17
        currentValue %= 256
    # print(key, value)
    # print(boxes)

totalPower = 0

for i in range(len(boxes)):
    for j in range(len(boxes[i])):
        totalPower += int(boxes[i][j][1]) * (i + 1) * (j + 1)

print(totalPower)