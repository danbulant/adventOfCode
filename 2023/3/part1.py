import re

symbols = []
partnumbers = []

lineNumber = 0
for line in open("./input2"):
    lineNumber += 1
    partnumbers += [(lineNumber, m.group(), m.start(), m.end()) for m in re.finditer(r'\d+', line)]
    symbols += [(lineNumber, m.group(), m.start(), m.end()) for m in re.finditer(r'[^\d.\n]', line)]

print(sum([
    int(partnumber[1]) for partnumber in partnumbers
    if len([symbol for symbol in symbols if abs(symbol[0] - partnumber[0]) <= 1 and partnumber[2] - 1 <= symbol[2] <= partnumber[3]]) > 0
]))